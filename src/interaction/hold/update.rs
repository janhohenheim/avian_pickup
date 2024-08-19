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
        Option<&PickupMass>,
        &Position,
        &Rotation,
    )>,
) {
    let max_error = 0.3048; // 12 inches in the source engine
    for (actor, config, grab, _shadow, holding, actor_rotation) in q_actor.iter_mut() {
        if grab.error > max_error {
            commands.entity(actor).add(SetVerb::new(Verb::Drop));
            continue;
        }
        let actor_pitch = actor_rotation.to_euler(EulerRot::YXZ).1;
        let _actor_to_prop_pitch = actor_pitch.clamp(config.min_pitch, config.max_pitch);
        let forward = Transform::from_rotation(actor_rotation.0).forward();
        collide_get_extent(forward);

        let prop = holding.0;
        let (preferred_rotation, preferred_distance, _pickup_mass, _position, rotation) =
            q_prop.get_mut(prop).unwrap();
        let _target_rotation = preferred_rotation
            .map(|preferred| preferred.0)
            .unwrap_or(rotation.0);
        let _target_distance = preferred_distance.copied().unwrap_or_default().0;
    }
}

fn collide_get_extent(dir: Dir3) {
    let collider = Collider::cuboid(1.0, 1.0, 1.0);
    let support_map = collider.shape().as_support_map().unwrap();
    let extent = support_map.support_point(&default(), &Vector::from(-dir).into());
    info!("extent: {extent:?}");
}
