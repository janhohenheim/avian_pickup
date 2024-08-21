use std::f32::consts::{PI, TAU};

use super::ShadowParams;
use crate::{prelude::*, verb::Holding};

/// CGrabController::Simulate
pub(super) fn set_velocities(
    time: Res<Time>,
    mut q_prop: Query<(
        &Name,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &Position,
        &Rotation,
    )>,
    mut q_actor: Query<(
        &Name,
        &ShadowParams,
        &Holding,
        &AvianPickupActor,
        &Position,
        &Rotation,
    )>,
) {
    // Valve uses CGrabController::Simulate, which does *a lot* of stuff,
    // but from testing, it seems like this does the job pretty much identically,
    // so I reverted to this simpler version. If you need the original version,
    // check out the commit aa51b2bc4dbc52049476135ba146b3ba143b681a
    let dt = time.delta_seconds();
    let inv_dt = dt.recip();
    for (actor_name, shadow, holding, actor, ac_position, ac_rotation) in q_actor.iter_mut() {
        let prop = holding.0;
        // Safety: All props are rigid bodies, so they are guaranteed to have a
        // `Position`, `Rotation`, `LinearVelocity`, and `AngularVelocity`.
        let (prop_name, mut velocity, mut angvel, position, rotation) =
            q_prop.get_mut(prop).unwrap();

        let delta_position = shadow.target_position - position.0;

        let delta_rotation = shadow.target_rotation * rotation.0.inverse();
        let (axis, angle) = delta_rotation.to_axis_angle();
        // This is needed because otherwise we will sometimes rotate the long way around
        let angle = if angle > PI { angle - TAU } else { angle };
        let delta_rotation_scaled_axis = axis * angle;

        // This is used for a bit of easing. We don't need to be careful about
        // things like overshooting as we are in a fixed timestep.
        // Negative because the dt is already inverted
        let vel_ease = 1.0;
        velocity.0 = (delta_position * inv_dt * vel_ease).clamp_length_max(shadow.max_speed);
        velocity.0 = zero_if_near_zero(velocity.0);

        let angvel_ease = f32::exp(-actor.angular_velocity_easing);
        angvel.0 = (delta_rotation_scaled_axis * inv_dt * angvel_ease)
            .clamp_length_max(shadow.max_angular);
        angvel.0 = zero_if_near_zero(angvel.0);

        info!(
            "{prop_name}: velocity: {}, angvel: {}",
            velocity.0, angvel.0
        );
        let pos = if position.x < 0.0 {
            Vec3 {
                x: position.x + 5.0,
                y: position.y,
                z: position.z,
            }
        } else {
            Vec3 {
                x: position.x - 5.0,
                y: position.y,
                z: position.z,
            }
        };
        info!(
            "{prop_name}: position: {}, rotation: {:?}",
            pos,
            rotation.0.to_euler(EulerRot::YXZ)
        );

        let ac_pos = if ac_position.x < 0.0 {
            Vec3 {
                x: ac_position.x + 5.0,
                y: ac_position.y,
                z: ac_position.z,
            }
        } else {
            Vec3 {
                x: ac_position.x - 5.0,
                y: ac_position.y,
                z: ac_position.z,
            }
        };

        info!(
            "{actor_name}: position: {}, rotation: {:?}",
            ac_pos,
            ac_rotation.0.to_euler(EulerRot::YXZ)
        );
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
