use std::f32::consts::TAU;

use crate::prelude::*;

mod on_hold;
mod simulate;
mod update;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_hold::on_hold);
    app.get_schedule_mut(PhysicsSchedule).unwrap().add_systems(
        (
            // Updates the error that `update_object` uses
            update::update_error,
            // Sets `error_time` to 0
            update::update_object,
            simulate::set_velocities,
        )
            .chain()
            .in_set(HandleVerbSystem::Hold),
    );
}

pub(super) mod prelude {
    pub(crate) use super::{GrabParams, ShadowParams};
}

#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct ShadowParams {
    /// Global target position of the held prop
    target_position: Vec3,
    /// Global target rotation of the held prop
    target_rotation: Quat,
    max_angular: f32,
    max_speed: f32,
}

impl Default for ShadowParams {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            max_angular: TAU * 10.0,
            max_speed: 25.4,
        }
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct GrabParams {
    /// Time until error starts accumulating
    error_time: f32,
    /// The distance between the object and the target position
    error: f32,
}

impl Default for GrabParams {
    fn default() -> Self {
        Self {
            error_time: 0.0,
            error: 0.0,
        }
    }
}
