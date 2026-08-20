use crate::prelude::*;
use avian3d::math::{Scalar, TAU};

pub(super) fn plugin(_app: &mut App) {}

/// Define what the [`AvianPickupActor`] is holding.
///
/// The held entity will have the [`HeldBy`] component.
///
/// You can add/remove it to start/stop the interaction,
/// same can be done with [`HeldBy`].
#[derive(Debug, Clone, Copy, Component)]
#[relationship(relationship_target = HeldBy)]
pub struct Holding(pub Entity);

impl Default for Holding {
    fn default() -> Self {
        Holding(Entity::PLACEHOLDER)
    }
}

/// Component for props that are held by an [`AvianPickupActor`].
///
/// Entity holding it will have the [`Holding`] component.
///
/// You can add/remove to start/stop the interaction,
/// same can be done with [`Holding`].
#[derive(Debug, Clone, Copy, Component)]
#[relationship_target(relationship = Holding)]
pub struct HeldBy(Entity);

impl Default for HeldBy {
    fn default() -> Self {
        HeldBy(Entity::PLACEHOLDER)
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct ShadowParams {
    /// Global target position of the held prop
    pub(crate) target_position: Vec3,
    /// Global target rotation of the held prop
    pub(crate) target_rotation: Quat,
    pub(crate) max_angular: Scalar,
    pub(crate) max_speed: Scalar,
}

impl Default for ShadowParams {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            // the following two are tuned by hand
            max_angular: TAU * 2.0,
            max_speed: 10.0,
        }
    }
}

/// Cache for accumulating errors when holding an object.
/// When this reaches a critical value, the object will be dropped.
#[derive(Debug, Copy, Clone, Component)]
pub(crate) struct HoldError {
    /// Time until error starts accumulating
    pub(crate) error_time: f32,
    /// The distance between the object and the target position
    pub(crate) error: f32,
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
