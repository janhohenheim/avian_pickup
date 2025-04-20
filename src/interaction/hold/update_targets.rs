use avian3d::sync::ancestor_marker::AncestorMarker;

use super::{HoldSystem, prelude::*};
use crate::{
    avian_util::get_rigid_body_colliders,
    math::{GetBestGlobalTransform as _, rigid_body_compound_collider},
    prelude::*,
    prop::PrePickupRotation,
    verb::{Holding, SetVerb, Verb},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, set_targets.in_set(HoldSystem::SetTargets));
}

/// CGrabController::UpdateObject
fn set_targets(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut q_actor: Query<(
        Entity,
        &AvianPickupActor,
        &HoldError,
        &mut ShadowParams,
        &Holding,
    )>,
    q_actor_transform: Query<(&GlobalTransform, Option<&Position>, Option<&Rotation>)>,
    mut q_prop: Query<(
        &Rotation,
        &GlobalTransform,
        &ComputedCenterOfMass,
        Option<&PrePickupRotation>,
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistanceOverride>,
        Option<&PitchRangeOverride>,
    )>,

    q_collider_ancestor: Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider_parent: Query<&ColliderParent>,
    mut q_collider: Query<(&GlobalTransform, &Collider, Option<&CollisionLayers>)>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, config, hold_error, mut shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        if hold_error.error > max_error {
            commands
                .entity(actor)
                .queue(SetVerb::new(Verb::Drop { prop, forced: true }));
            continue;
        }
        let actor_transform = q_actor_transform.get_best_global_transform(actor);

        let Ok((
            prop_rotation,
            prop_transform,
            prop_center_of_mass,
            pre_pickup_rotation,
            preferred_rotation,
            preferred_distance,
            clamp_pitch,
        )) = q_prop.get_mut(prop)
        else {
            error!("Prop entity was deleted or in an invalid state. Ignoring.");
            continue;
        };
        let pitch_range = clamp_pitch
            .map(|c| &c.0)
            .unwrap_or(&config.hold.pitch_range);
        let (actor_yaw, actor_pitch, actor_roll) = actor_transform.rotation.to_euler(EulerRot::YXZ);
        let actor_to_prop_pitch = actor_pitch.clamp(*pitch_range.start(), *pitch_range.end());
        let clamped_rotation =
            Quat::from_euler(EulerRot::YXZ, actor_yaw, actor_to_prop_pitch, actor_roll);
        let forward = Transform::from_rotation(clamped_rotation).forward();
        // We can't cast a ray wrt an entire rigid body out of the box,
        // so we manually collect all colliders in the hierarchy and
        // construct a compound collider.
        let colliders = get_rigid_body_colliders(
            prop,
            &q_collider_ancestor,
            &q_collider.transmute_lens().query(),
        );
        let prop_collider =
            rigid_body_compound_collider(colliders.as_deref(), &q_collider, &config.prop_filter);
        let Some(prop_collider) = prop_collider else {
            error!("Held prop does not have a collider in its hierarchy. Ignoring.");
            continue;
        };
        let prop_radius_wrt_direction =
            collider_get_extent(&prop_collider, prop_rotation.0, -forward);
        let Some(prop_radius_wrt_direction) = prop_radius_wrt_direction else {
            error!(
                "Failed to get collider extent: Parry failed to find a hit with its AABB. Ignoring prop."
            );
            continue;
        };

        let min_non_penetrating_distance = prop_radius_wrt_direction;
        let min_distance = min_non_penetrating_distance + config.hold.min_distance;
        // The 2013 code now additionally does `min_distance = (min_distance * 2) + 24
        // inches` That seems straight up bizarre, so I refuse to do that.
        let preferred_distance = preferred_distance
            .map(|d| d.0)
            .unwrap_or(config.hold.preferred_distance)
            + min_non_penetrating_distance;
        // The 2013 code does `max_distance = preferred_distance + min_distance`
        // which means that `preferred_distance` is the distance between the prop's
        // edge and the actors's edge. Expect psyche, actually `min_distance` gets
        // deduced again at some point!
        let max_distance = preferred_distance.max(min_distance);
        let Some(actor_space_rotation) = preferred_rotation
            .map(|preferred| preferred.0)
            .or_else(|| pre_pickup_rotation.map(|pre| pre.0))
        else {
            error!("Held prop does not have a preferred or pre-pickup rotation. Ignoring.");
            continue;
        };
        // orient the prop wrt the actor
        // The 2013 code uses the non-clamped code here, resulting in the prop
        // rotating when looking further up than the clamp allows.
        // Looks weird imo, so we use the clamped rotation.
        let clamped_actor_transform = actor_transform.with_rotation(clamped_rotation);
        let target_rotation =
            prop_rotation_from_actor_space(actor_space_rotation, clamped_actor_transform);

        shadow.target_rotation = target_rotation;

        // The cast needs to be longer to account for the fact that
        // the prop might hit terrain with the side that is not facing
        // the player. We are assuming the prop has the same radius
        // "behind" it as it has in front of it. Also add a bit of
        // padding to be safe.
        let max_cast_toi = max_distance + min_distance + 0.5;

        // Not filtering this out later because we want the cast to "pass through" the
        // prop to get the distance to the terrain behind it.
        let is_terrain = |entity: Entity| {
            q_collider_parent
                .get(entity)
                .is_ok_and(|parent| parent.get() != prop)
        };

        let global_center_of_mass = prop_transform.transform_point(prop_center_of_mass.0);
        let terrain_hit = spatial_query.cast_shape_predicate(
            &prop_collider,
            global_center_of_mass,
            target_rotation,
            forward,
            &ShapeCastConfig {
                max_distance: max_cast_toi,
                ignore_origin_penetration: true,
                ..default()
            },
            &config.obstacle_filter,
            &is_terrain,
        );
        let distance = if let Some(terrain_hit) = terrain_hit {
            let toi = terrain_hit.distance;
            let fraction = toi / max_distance;
            if fraction < 0.5 {
                // not doing `max(min_distance, toi)` here because that would
                // result in the prop being too close to the player
                // better to intersect with the terrain than to the player.
                min_distance
            } else {
                max_distance.min(toi)
            }
        } else {
            max_distance
        };
        // Pretty sure we don't need to go through the CalcClosestPointOnLine song and
        // dance since we already have made sure that the prop has a sensible minimum
        // distance
        let target_position = actor_transform.translation + forward * distance;
        // target_position is pointing to the origin of the prop, which is often at its "feet".
        // This looks really weird when holding, so let's hold it at the center of mass instead.
        // Note that the following calculation is distinct from just `prop_center_of_mass.0`,
        // as that one would be the offset if the prop had no rotation.
        let center_of_mass_offset = global_center_of_mass - prop_transform.translation();
        shadow.target_position = target_position - center_of_mass_offset;
    }
}

/// The original code gets the support point of the collider in the direction,
/// but we can only do that for convex shapes in parry. Notably, compound shapes
/// made of convex shapes are not supported.\
/// So, we instead just cast a ray in the direction and get the hit point.
/// Since the original code multiplies the direction by the dot product of
/// the direction and the support point, it looks like the result is the same.
/// That's why we just return the TOI directly.
/// Note that we just use the AABB of the compound collider here, which is
/// not the exact convex hull, but should be close enough.
fn collider_get_extent(collider: &Collider, rotation: Quat, dir: Dir3) -> Option<f32> {
    let aabb = collider.aabb(Vec3::ZERO, Quat::IDENTITY);
    let aabb_lengths = aabb.size();
    let aabb_collider = Collider::cuboid(aabb_lengths.x, aabb_lengths.y, aabb_lengths.z);

    const TRANSLATION: Vec3 = Vec3::ZERO;
    const RAY_ORIGIN: Vec3 = Vec3::ZERO;
    // We cast from inside the collider, so we don't care about a max TOI
    const MAX_TOI: f32 = f32::INFINITY;
    // Needs to be false to not just get the origin back
    const SOLID: bool = false;

    let hit = aabb_collider.cast_ray(
        TRANSLATION,
        rotation,
        RAY_ORIGIN,
        dir.into(),
        MAX_TOI,
        SOLID,
    );
    hit.map(|(toi, _normal)| toi)
}

/// TransformAnglesFromPlayerSpace
fn prop_rotation_from_actor_space(rot: Quat, actor: Transform) -> Quat {
    let actor_matrix = actor.compute_affine();
    let rot_to_actor = Transform::from_rotation(rot).compute_affine();
    let out_affine = actor_matrix * rot_to_actor;
    Quat::from_affine3(&out_affine)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collide_get_extent() {
        let collider = Collider::capsule(0.3, 1.2);
        let rot = Quat::from_euler(EulerRot::YXZ, -0.014999974, -0.07314853, 0.);
        let dir = Vec3::new(0.014959301, -0.073083326, -0.9972137)
            .try_into()
            .unwrap();
        let extent = collider_get_extent(&collider, rot, dir).unwrap();
        assert!(extent > 0.0);
        assert!(extent < 0.6);
    }

    #[test]
    #[ignore = "Parry bug"]
    fn test_collider_get_extent_manual() {
        let collider = Collider::capsule(0.3, 1.2);
        let rotation = Quat::from_euler(EulerRot::YXZ, -0.014999974, -0.07314853, 0.);
        let dir = Vec3::new(0.014959301, -0.073083326, -0.9972137);

        const TRANSLATION: Vec3 = Vec3::ZERO;
        const ORIGIN: Vec3 = Vec3::ZERO;
        // We cast from inside the collider, so we don't care about a max TOI
        const MAX_TOI: f32 = f32::INFINITY;
        // Needs to be false to not just get the origin back
        const SOLID: bool = false;
        let hit = collider.cast_ray(TRANSLATION, rotation, ORIGIN, dir, MAX_TOI, SOLID);
        assert!(hit.is_some());
    }
}
