use avian3d::sync::ancestor_marker::AncestorMarker;
use bevy::prelude::*;

use super::{HoldError, ShadowParams};
use crate::{
    math::{rigid_body_compound_collider, GetBestGlobalTransform as _},
    prelude::*,
    prop::PrePickupRotation,
    verb::{Holding, SetVerb, Verb},
};

/// CGrabController::ComputeError(),
pub(super) fn update_error(
    time: Res<Time>,
    q_prop: Query<&Position>,
    mut q_actor: Query<(&mut HoldError, &ShadowParams, &Holding)>,
) {
    let dt = time.delta_seconds();
    for (mut hold_error, shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        hold_error.error_time += dt;
        if hold_error.error_time <= 0.0 {
            continue;
        }
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`.
        let position = q_prop.get(prop).unwrap();
        let mut error = (position.0 - shadow.target_position).length();
        if hold_error.error_time > 1.0 {
            hold_error.error_time = 1.0;
        }
        let speed = error / hold_error.error_time;
        if speed > shadow.max_speed {
            // this seems like it would still result in a speed above max_speed
            // but idk.
            error *= 0.5;
        }
        hold_error.error = hold_error.error.lerp(error, hold_error.error_time);
        hold_error.error_time = 0.0;
    }
}

/// CGrabController::UpdateObject
pub(super) fn update_object(
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
        Option<&PrePickupRotation>,
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&ClampPickupPitch>,
    )>,

    q_collider_ancestor: Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: Query<(&Transform, &Collider, Option<&CollisionLayers>)>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, config, hold_error, mut shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        if hold_error.error > max_error {
            commands.entity(actor).add(SetVerb::new(Verb::Drop(prop)));
            continue;
        }
        let actor_transform = q_actor_transform.get_best_global_transform(actor);

        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Rotation`.
        let (
            prop_rotation,
            pre_pickup_rotation,
            preferred_rotation,
            preferred_distance,
            clamp_pitch,
        ) = q_prop.get_mut(prop).unwrap();
        let clamp_pitch = clamp_pitch.copied().unwrap_or_default();

        let (actor_yaw, actor_pitch, actor_roll) = actor_transform.rotation.to_euler(EulerRot::YXZ);
        let actor_to_prop_pitch = actor_pitch.clamp(clamp_pitch.min, clamp_pitch.max);
        let clamped_rotation =
            Quat::from_euler(EulerRot::YXZ, actor_yaw, actor_to_prop_pitch, actor_roll);
        let forward = Transform::from_rotation(clamped_rotation).forward();
        // We can't cast a ray wrt an entire rigid body out of the box,
        // so we manually collect all colliders in the hierarchy and
        // construct a compound collider.
        let prop_collider = rigid_body_compound_collider(
            prop,
            &q_collider_ancestor,
            &q_collider,
            &config.prop_filter,
        );
        let Some(prop_collider) = prop_collider else {
            error!("Held prop does not have a collider in its hierarchy. Ignoring.");
            continue;
        };
        let prop_radius_wrt_direction =
            collide_get_extent(&prop_collider, Vec3::ZERO, prop_rotation.0, -forward);
        let actor_collider = rigid_body_compound_collider(
            actor,
            &q_collider_ancestor,
            &q_collider,
            &config.actor_filter,
        );
        let actor_radius_wrt_direction = if let Some(actor_collider) = actor_collider {
            let min_distance_to_not_penetrate = collide_get_extent(
                &actor_collider,
                Vec3::ZERO,
                actor_transform.rotation,
                forward,
            );
            min_distance_to_not_penetrate.max(config.min_distance)
        } else {
            config.min_distance
        };

        let min_distance = prop_radius_wrt_direction + actor_radius_wrt_direction;
        // The 2013 code now additionally does `min_distance = (min_distance * 2) + 24
        // inches` That seems straight up bizarre, so I refuse to do that.
        let preferred_distance = preferred_distance.copied().unwrap_or_default().0;
        // The 2013 code does `max_distance = preferred_distance + min_distance`
        // which means that `preferred_distance` is the distance between the prop's
        // edge and the actors's edge. Expect psyche, actually `min_distance` gets
        // deduced again at some point! I think it's more intuitive to have the
        // preferred distance be the distance between the prop's and
        // actor's origins if possible instead.
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

        let terrain_hit = spatial_query.cast_shape(
            &prop_collider,
            actor_transform.translation,
            target_rotation,
            forward,
            max_cast_toi,
            true,
            &config.obstacle_filter,
        );
        let distance = if let Some(terrain_hit) = terrain_hit {
            let toi = terrain_hit.time_of_impact;
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
        shadow.target_position = target_position;
    }
}

/// The original code gets the support point of the collider in the direction,
/// but we can only do that for convex shapes in parry. Notably, compound shapes
/// made of convex shapes are not supported.\
/// So, we instead just cast a ray in the direction and get the hit point.
/// Since the original code multiplies the direction by the dot product of
/// the direction and the support point, it looks like the result is the same.
/// That's why we just return the TOI directly.
fn collide_get_extent(collider: &Collider, origin: Vec3, rotation: Quat, dir: Dir3) -> f32 {
    const TRANSLATION: Vec3 = Vec3::ZERO;
    // We cast from inside the collider, so we don't care about a max TOI
    const MAX_TOI: f32 = f32::INFINITY;
    // Needs to be false to not just get the origin back
    const SOLID: bool = false;

    // This should not be necessary, but it seems like a parry
    // bug sometimes causes the hit to be `None` even though that should
    // be impossible: https://discord.com/channels/691052431525675048/1124043933886976171/1275214643341561970
    const ARBITRARY_ROTATION: f32 = 5e-3;
    for offset in [
        Quat::IDENTITY,
        Quat::from_rotation_x(ARBITRARY_ROTATION),
        Quat::from_rotation_x(-ARBITRARY_ROTATION),
        Quat::from_rotation_y(ARBITRARY_ROTATION),
        Quat::from_rotation_y(-ARBITRARY_ROTATION),
        Quat::from_rotation_z(ARBITRARY_ROTATION),
        Quat::from_rotation_z(-ARBITRARY_ROTATION),
    ] {
        let rotation = rotation * offset;
        let hit = collider.cast_ray(TRANSLATION, rotation, origin, dir.into(), MAX_TOI, SOLID);
        if let Some((toi, _normal)) = hit {
            return toi;
        }
    }

    // Absolute last resort: just fall back to the AABB's longest extent.
    // This *must* work, but it's longer than necessary and expensive.
    let aabb = collider.aabb(origin, rotation);

    (aabb.max / 2.).length()
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
        let extent = collide_get_extent(&collider, Vec3::ZERO, rot, dir);
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
