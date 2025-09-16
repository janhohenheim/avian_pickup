use bevy_time::Time;

use super::{HoldSystem, prelude::*};
use crate::{prelude::*, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        update_error.in_set(HoldSystem::UpdateError),
    );
}

/// CGrabController::ComputeError(),
pub fn update_error(
    time: Res<Time>,
    q_prop: Query<&GlobalTransform>,
    mut q_actor: Query<(&mut HoldError, &ShadowParams, &Holding)>,
) {
    let dt = time.delta_secs();
    for (mut hold_error, shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        hold_error.error_time += dt;
        if hold_error.error_time <= 0.0 {
            continue;
        }
        let Ok(prop_transform) = q_prop.get(prop) else {
            error!("Prop entity was deleted or in an invalid state. Ignoring.");
            continue;
        };
        let mut error = (prop_transform.translation() - shadow.target_position).length();
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
