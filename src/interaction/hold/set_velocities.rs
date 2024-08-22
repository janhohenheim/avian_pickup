use std::f32::consts::{PI, TAU};

use super::{prelude::ShadowParams, HoldSystem};
use crate::{prelude::*, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        set_velocities.in_set(HoldSystem::SetVelocities),
    );
}

/// CGrabController::Simulate
fn set_velocities(
    time: Res<Time>,
    mut q_prop: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &Position,
        &Rotation,
    )>,
    mut q_actor: Query<(&ShadowParams, &Holding, &AvianPickupActor)>,
) {
    // Valve uses CGrabController::Simulate, which does *a lot* of stuff,
    // but from testing, it seems like this does the job pretty much identically,
    // so I reverted to this simpler version. If you need the original version,
    // check out the commit aa51b2bc4dbc52049476135ba146b3ba143b681a
    let dt = time.delta_seconds();
    let inv_dt = dt.recip();
    for (shadow, holding, actor) in q_actor.iter_mut() {
        let prop = holding.0;
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`, `Rotation`, `LinearVelocity`, and `AngularVelocity`.
        let (mut velocity, mut angvel, position, rotation) = q_prop.get_mut(prop).unwrap();

        let delta_position = shadow.target_position - position.0;

        let delta_rotation = shadow.target_rotation * rotation.0.inverse();
        let (axis, angle) = delta_rotation.to_axis_angle();
        // This is needed because otherwise we will sometimes rotate the long way around
        let angle = if angle > PI { angle - TAU } else { angle };
        let delta_rotation_scaled_axis = axis * angle;

        // This is used for a bit of easing. We don't need to be careful about
        // things like overshooting as we are in a fixed timestep.
        // Negative because the dt is already inverted
        let vel_ease = f32::exp(-actor.linear_velocity_easing);
        velocity.0 = (delta_position * inv_dt * vel_ease).clamp_length_max(shadow.max_speed);
        velocity.0 = zero_if_near_zero(velocity.0);

        let angvel_ease = f32::exp(-actor.angular_velocity_easing);
        angvel.0 = (delta_rotation_scaled_axis * inv_dt * angvel_ease)
            .clamp_length_max(shadow.max_angular);
        angvel.0 = zero_if_near_zero(angvel.0);
    }
}

fn zero_if_near_zero(vec: Vec3) -> Vec3 {
    // This seems large, but since we multiply by the inverse of the delta time,
    // it's actually quite small.
    let arbitrary_cutoff = 1e-4;
    if vec.length_squared() < arbitrary_cutoff {
        Vec3::ZERO
    } else {
        vec
    }
}
