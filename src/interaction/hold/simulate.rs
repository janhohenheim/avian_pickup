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
        let (mut velocity, mut angvel, position, rotation) = q_prop.get_mut(prop).unwrap();

        let delta_position = shadow.target_position - position.0;
        let delta_rotation = shadow.target_rotation * rotation.0.inverse();

        velocity.0 = delta_position * inv_dt;
        if velocity.0.length_squared() > (shadow.max_speed * shadow.max_speed) {
            velocity.0 = velocity.0.normalize_or_zero() * shadow.max_speed;
        }
        angvel.0 = delta_rotation.to_scaled_axis() * inv_dt;
        if angvel.0.length_squared() > (shadow.max_angular * shadow.max_angular) {
            angvel.0 = angvel.0.normalize_or_zero() * shadow.max_angular;
        }
    }
}
