use std::f32::consts::{PI, TAU};

use super::ShadowParams;
use crate::{prelude::*, verb::Holding};

/// CGrabController::Simulate
pub(super) fn set_velocities(
    time: Res<Time>,
    mut q_prop: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &Position,
        &Rotation,
    )>,
    mut q_actor: Query<(&ShadowParams, &Holding)>,
) {
    // Valve uses CGrabController::Simulate, which does *a lot* of stuff,
    // but from testing, it seems like this does the job pretty much identically,
    // so I reverted to this simpler version. If you need the original version,
    // check out the commit aa51b2bc4dbc52049476135ba146b3ba143b681a
    let dt = time.delta_seconds();
    let inv_dt = dt.recip();
    for (shadow, holding) in q_actor.iter_mut() {
        let prop = holding.0;
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`, `Rotation`, `LinearVelocity`, and `AngularVelocity`.
        let (mut velocity, mut angvel, position, rotation) = q_prop.get_mut(prop).unwrap();

        let delta_position = shadow.target_position - position.0;
        info!("position: {:?}", position.0);

        let delta_rotation = shadow.target_rotation * rotation.0.inverse();
        let (axis, angle) = delta_rotation.to_axis_angle();
        // This is needed because otherwise we will sometimes rotate the long way around
        let angle = if angle > PI { angle - TAU } else { angle };
        let delta_rotation_scaled_axis = axis * angle;

        velocity.0 = (delta_position * inv_dt).clamp_length_max(shadow.max_speed);
        angvel.0 = (delta_rotation_scaled_axis * inv_dt).clamp_length_max(shadow.max_angular);
    }
}
