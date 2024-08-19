use avian3d::math::Vector;
use bevy::prelude::*;

use super::{GrabParams, ShadowParams};
use crate::{
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
        &Rotation,
    )>,
    mut q_prop: Query<(
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&ClampPickupPitch>,
        &Position,
        &Rotation,
    )>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, _config, grab, _shadow, holding, actor_rotation) in q_actor.iter_mut() {
        if grab.error > max_error {
            commands.entity(actor).add(SetVerb::new(Verb::Drop));
            continue;
        }

        let prop = holding.0;
        let (preferred_rotation, preferred_distance, clamp_pitch, _position, rotation) =
            q_prop.get_mut(prop).unwrap();
        let clamp_pitch = clamp_pitch.copied().unwrap_or_default();

        let actor_pitch = actor_rotation.to_euler(EulerRot::YXZ).1;
        let _actor_to_prop_pitch = actor_pitch.clamp(clamp_pitch.min, clamp_pitch.max);
        let forward = Transform::from_rotation(actor_rotation.0).forward();
        //let radial = collide_get_extent(forward);

        let _target_rotation = preferred_rotation
            .map(|preferred| preferred.0)
            .unwrap_or(rotation.0);
        let _target_distance = preferred_distance.copied().unwrap_or_default().0;
    }
}

/// The original code gets the support point of the collider in the direction,
/// but we can only do that for convex shapes in parry. Notably, compound shapes
/// made of convex shapes are not supported.\
/// So, we instead just cast a ray in the direction and get the hit point.
fn collide_get_extent(collider: &Collider, origin: Vec3, rotation: Quat, dir: Dir3) -> Vec3 {
    const TRANSLATION: Vec3 = Vec3::ZERO;
    // We cast from inside the collider, so we don't care about a max TOI
    const MAX_TOI: f32 = f32::MAX;
    // Needs to be false to not just get the origin back
    const SOLID: bool = false;
    let hit = collider.cast_ray(TRANSLATION, rotation, origin, dir.into(), MAX_TOI, SOLID);
    let (toi, _normal) = hit.expect("Casting a ray from inside a collider did not hit the collider itself. This seems like a bug in Avian.");
    dir * toi
}
