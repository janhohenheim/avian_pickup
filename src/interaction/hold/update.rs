use bevy::prelude::*;

use super::{GrabParams, ShadowParams};
use crate::{prelude::*, verb::Holding};

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
    mut q_prop: Query<(
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&PickupMass>,
        &Position,
        &Rotation,
    )>,
    mut q_actor: Query<(
        &AvianPickupActorState,
        &mut GrabParams,
        &ShadowParams,
        &Holding,
    )>,
) {
    let _max_error = 0.3048; // 12 inches in the source engine
    for (_state, _grab, _shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        let (preferred_rotation, preferred_distance, _pickup_mass, _position, rotation) =
            q_prop.get_mut(prop).unwrap();
        let _target_rotation = preferred_rotation
            .map(|preferred| preferred.0)
            .unwrap_or(rotation.0);
        let _target_distance = preferred_distance.copied().unwrap_or_default().0;
    }
}
