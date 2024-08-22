use std::f32::consts::TAU;

use crate::prelude::*;

mod on_hold;
mod set_velocities;
mod shadow_params;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        PhysicsSchedule,
        (HoldSystem::UpdateShadowParams, HoldSystem::SetVelocities)
            .chain()
            .in_set(HandleVerbSystem::Hold),
    )
    .add_plugins((
        on_hold::plugin,
        shadow_params::plugin,
        set_velocities::plugin,
    ));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum HoldSystem {
    UpdateShadowParams,
    SetVelocities,
}

pub(super) mod prelude {
    pub(crate) use super::{HoldError, ShadowParams};
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

/// Cache for accumulating errors when holding an object.
/// When this reaches a critical value, the object will be dropped.
#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct HoldError {
    /// Time until error starts accumulating
    error_time: f32,
    /// The distance between the object and the target position
    error: f32,
}

impl HoldError {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for HoldError {
    fn default() -> Self {
        Self {
            // 1 second until error starts accumulating
            error_time: -1.0,
            error: 0.0,
        }
    }
}
