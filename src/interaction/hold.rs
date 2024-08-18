use std::f32::consts::TAU;

use crate::prelude::*;

mod on_hold;
mod simulate;
mod update;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule).unwrap().add_systems(
        // TODO: idk about the order
        (simulate::simulate, on_hold::on_hold)
            .chain()
            .in_set(AvianPickupSystem::HandleVerb),
    );
}

pub(super) mod prelude {
    pub(crate) use super::{GrabParams, ShadowParams};
}

#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct ShadowParams {
    target_position: Vec3,
    target_rotation: Quat,
    max_angular: f32,
    max_damp_angular: f32,
    max_speed: f32,
    max_damp_speed: f32,
    // damp_factor = 1
    // teleport_distance = 0
}

impl Default for ShadowParams {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            max_angular: TAU * 10.0,
            max_damp_angular: 0.0,
            max_speed: 25.4,
            max_damp_speed: 25.4 * 2.,
        }
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct GrabParams {
    contact_amount: f32,
    time_to_arrive: f32,
    /// Time until error starts accumulating
    error_time: f32,
    /// The distance between the object and the target position
    error: f32,
}

impl Default for GrabParams {
    fn default() -> Self {
        Self {
            contact_amount: 0.0,
            time_to_arrive: 0.0,
            error_time: 0.0,
            error: 0.0,
        }
    }
}
