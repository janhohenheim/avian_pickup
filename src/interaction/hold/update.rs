use bevy::prelude::*;

use super::{GrabParams, ShadowParams};
use crate::prelude::*;

/// CGrabController::ComputeError(),
pub(super) fn update_error(
    q_prop: Query<&Position>,
    mut q_actor: Query<(&AvianPickupActorState, &mut GrabParams, &ShadowParams)>,
) {
    for (&state, mut grab, shadow) in q_actor.iter_mut() {
        let AvianPickupActorState::Holding(prop_entity) = state else {
            continue;
        };
        if grab.error_time <= 0.0 {
            continue;
        }
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`.
        let position = q_prop.get(prop_entity).unwrap();
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
    _q_prop: Query<&Position>,
    _q_actor: Query<(&AvianPickupActorState, &mut GrabParams, &ShadowParams)>,
) {
    let _max_error = 0.3048; // 12 inches in the source engine
}
