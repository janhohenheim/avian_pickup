use avian3d::sync::ancestor_marker::AncestorMarker;
use bevy::prelude::*;

use super::{GrabParams, ShadowParams};
use crate::{
    math::rigid_body_compound_collider,
    prelude::*,
    verb::{Holding, SetVerb, Verb},
};

/// CGrabController::ComputeError(),
pub(super) fn update_error(
    q_prop: Query<&Position>,
    mut q_actor: Query<(&mut GrabParams, &ShadowParams, &Holding)>,
) {
    for (mut grab, shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        if grab.error_time <= 0.0 {
            continue;
        }
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`.
        let position = q_prop.get(prop).unwrap();
        let mut error = (position.0 - shadow.target_position).length();
        if grab.error_time > 1.0 {
            grab.error_time = 1.0;
        }
        let speed = error / grab.error_time;
        if speed > shadow.max_speed {
            // this seems like it would still result in a speed above max_speed
            // but idk.
            error *= 0.5;
        }
        grab.error = grab.error.lerp(error, grab.error_time);
        grab.error_time = 0.0;
    }
}

/// CGrabController::UpdateObject
pub(super) fn update_object(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut q_actor: Query<(
        Entity,
        &AvianPickupActor,
        &GrabParams,
        &mut ShadowParams,
        &Holding,
        &GlobalTransform,
    )>,
    mut q_prop: Query<(
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&ClampPickupPitch>,
        &Rotation,
    )>,

    q_collider_ancestor: Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: Query<(&Transform, &Collider), Without<Sensor>>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, config, grab, mut shadow, holding, actor_transform) in q_actor.iter_mut() {
        if grab.error > max_error {
            commands.entity(actor).add(SetVerb::new(Verb::Drop));
            continue;
        }
        let actor_transform = actor_transform.compute_transform();

        let prop = holding.0;
        let (preferred_rotation, preferred_distance, clamp_pitch, prop_rotation) =
            q_prop.get_mut(prop).unwrap();
        let clamp_pitch = clamp_pitch.copied().unwrap_or_default();

        let (actor_yaw, actor_pitch, actor_roll) = actor_transform.rotation.to_euler(EulerRot::YXZ);
        let actor_to_prop_pitch = actor_pitch.clamp(clamp_pitch.min, clamp_pitch.max);
        let clamped_rotation =
            Quat::from_euler(EulerRot::YXZ, actor_yaw, actor_to_prop_pitch, actor_roll);
        let forward = Transform::from_rotation(clamped_rotation).forward();
        // We can't cast a ray wrt an entire rigid body out of the box,
        // so we manually collect all colliders in the hierarchy and
        // construct a compound collider.
        let prop_collider = rigid_body_compound_collider(prop, &q_collider_ancestor, &q_collider);
        let Some(prop_collider) = prop_collider else {
            error!("Held prop does not have a collider in its hierarchy. Ignoring.");
            continue;
        };
        let prop_radius_wrt_direction =
            collide_get_extent(&prop_collider, Vec3::ZERO, prop_rotation.0, -forward);
        let actor_collider = rigid_body_compound_collider(actor, &q_collider_ancestor, &q_collider);
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

        let terrain_hit = spatial_query.cast_ray(
            actor_transform.translation,
            forward,
            max_distance,
            true,
            &config.terrain_filter,
        );
        let distance = if let Some(terrain_hit) = terrain_hit {
            let fraction = terrain_hit.time_of_impact / max_distance;
            if fraction < 0.5 {
                min_distance
            } else {
                max_distance
            }
        } else {
            max_distance
        };
        // Pretty sure we don't need to go through the CalcClosestPointOnLine song and
        // dance since we already have made sure that the prop has a sensible minimum
        // distance
        let target_position = actor_transform.translation + forward * distance;

        // TODO:
        // - Apply this rotation relative to the player
        // - Don't fall back to the current, but the initial rotation
        let target_rotation = preferred_rotation
            .map(|preferred| preferred.0)
            .unwrap_or(prop_rotation.0);

        shadow.target_position = target_position;
        shadow.target_rotation = target_rotation;
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
    let hit = collider.cast_ray(TRANSLATION, rotation, origin, dir.into(), MAX_TOI, SOLID);

    if let Some((toi, _normal)) = hit {
        toi
    } else {
        // This should not be necessary, but it seems like a parry
        // bug sometimes causes the hit to be `None` even though that should
        // be impossible: https://discord.com/channels/691052431525675048/1124043933886976171/1275214643341561970
        let aabb = collider.aabb(origin, rotation);
        let longest_extent = (aabb.max / 2.).length();
        longest_extent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_collide_get_extent() {
        let collider = Collider::capsule(0.3, 1.2);
        let rot = Quat::from_euler(EulerRot::YXZ, -0.014999974, -0.07314853, 0.);
        let dir = Vec3::new(0.014959301, -0.073083326, -0.9972137)
            .try_into()
            .unwrap();
        let extent = collide_get_extent(&collider, Vec3::ZERO, rot, dir);
        assert_eq!(extent, 0.3);
    }

    #[test]
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
