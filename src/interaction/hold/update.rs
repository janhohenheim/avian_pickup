use avian3d::{math::Vector, sync::ancestor_marker::AncestorMarker};
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
    mut q_actor: Query<(
        Entity,
        &AvianPickupActor,
        &mut GrabParams,
        &ShadowParams,
        &Holding,
        &Position,
        &Rotation,
    )>,
    mut q_prop: Query<(
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&ClampPickupPitch>,
        &Position,
        &Rotation,
    )>,

    q_collider_ancestor: Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: Query<(&Transform, &Collider), Without<Sensor>>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, _config, grab, _shadow, holding, actor_position, actor_rotation) in
        q_actor.iter_mut()
    {
        if grab.error > max_error {
            commands.entity(actor).add(SetVerb::new(Verb::Drop));
            continue;
        }

        let prop = holding.0;
        let (preferred_rotation, preferred_distance, clamp_pitch, prop_position, prop_rotation) =
            q_prop.get_mut(prop).unwrap();
        let clamp_pitch = clamp_pitch.copied().unwrap_or_default();

        let actor_pitch = actor_rotation.to_euler(EulerRot::YXZ).1;
        let _actor_to_prop_pitch = actor_pitch.clamp(clamp_pitch.min, clamp_pitch.max);
        let forward = Transform::from_rotation(actor_rotation.0).forward();
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
        let Some(actor_collider) = actor_collider else {
            error!("AvianPickupActor does not have a collider in its hierarchy. Ignoring.");
            continue;
        };
        let actor_radius_wrt_direction =
            collide_get_extent(&actor_collider, Vec3::ZERO, actor_rotation.0, forward);

        let min_distance = prop_radius_wrt_direction + actor_radius_wrt_direction;
        // The 2013 code now additionally does `min_distance = (min_distance * 2) + 24
        // inches` That seems straight up bizarre, so I refuse to do that.
        let preferred_distance = preferred_distance.copied().unwrap_or_default().0;
        // The 2013 code does `distance = preferred_distance + min_distance``
        // which means that `preferred_distance` is the distance between the prop's
        // edge and the actors's edge. Not wrong, but I think it's more intuitive
        // to have the preferred distance be the distance between the prop's and
        // actor's origins.
        let distance = preferred_distance.max(min_distance);

        let _target_rotation = preferred_rotation
            .map(|preferred| preferred.0)
            .unwrap_or(prop_rotation.0);
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
    let (toi, _normal) = hit.expect(
        "Casting a ray from inside a collider did not hit the collider itself.\n\
        This means the compound collider we constructed is malformed.\n\
        This is a bug. Please report it on `avian_pickup`s GitHub page.",
    );
    toi
}
