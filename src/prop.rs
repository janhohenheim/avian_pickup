//! Components that can be placed on props to customize their behavior when
//! picked up or thrown. All of these are optional.
use std::ops::RangeInclusive;

use crate::prelude::*;
use avian3d::{math::Scalar, prelude::Mass};
use bevy_ecs::prelude::*;

use crate::prelude::AvianPickupActor;

pub(super) fn plugin(_app: &mut App) {}

pub(super) mod prelude {
    pub use super::{
        HeldProp, PickupMassOverride, PitchRangeOverride, PreferredPickupDistanceOverride,
        PreferredPickupRotation, ThrownAngularSpeedOverride, ThrownLinearSpeedOverride,
    };
}

/// Insert this on an object to set its rotation when picked up.
/// The rotation is in the actor's local space, i.e. the prop will rotate along
/// with the actor in order to maintain this rotation.\
/// Useful for e.g. making sure that a telephone you pick up is always held
/// facing the player.
///
/// If an object has no `PreferredPickupRotation`, it will be held with whatever
/// rotation it had when picked up.
#[derive(Debug, Clone, PartialEq, Component, Default, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PreferredPickupRotation(pub Quat);

#[derive(Debug, Clone, PartialEq, Component)]
pub(crate) struct PrePickupRotation(pub Quat);

/// Insert this on a prop to override
/// [`AvianPickupActorHoldConfig::pitch_range`](crate::prelude::AvianPickupActorHoldConfig::pitch_range).
#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PitchRangeOverride(pub RangeInclusive<Scalar>);
impl Default for PitchRangeOverride {
    fn default() -> Self {
        Self(AvianPickupActor::default().hold.pitch_range)
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActorHoldConfig::preferred_distance`](crate::prelude::AvianPickupActorHoldConfig::preferred_distance).
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PreferredPickupDistanceOverride(pub Scalar);

impl Default for PreferredPickupDistanceOverride {
    fn default() -> Self {
        Self(AvianPickupActor::default().hold.preferred_distance)
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActorPullConfig::max_prop_mass`](crate::prelude::AvianPickupActorPullConfig::max_prop_mass).
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PickupMassOverride(pub Scalar);

impl Default for PickupMassOverride {
    fn default() -> Self {
        Self(AvianPickupActor::default().hold.temporary_prop_mass)
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActorThrowConfig::linear_speed_range`](crate::prelude::AvianPickupActorThrowConfig::linear_speed_range).
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct ThrownLinearSpeedOverride(pub Scalar);

impl Default for ThrownLinearSpeedOverride {
    fn default() -> Self {
        Self(*AvianPickupActor::default().throw.linear_speed_range.end())
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActorThrowConfig::angular_speed_range`](crate::prelude::AvianPickupActorThrowConfig::angular_speed_range).
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct ThrownAngularSpeedOverride(pub Scalar);

impl Default for ThrownAngularSpeedOverride {
    fn default() -> Self {
        Self(*AvianPickupActor::default().throw.angular_speed_range.end())
    }
}

/// The cached mass that an object had before it was picked up
/// that will be restored again when it is dropped.
/// In other words, this is the mass before and after the pickup.
/// Only used if the object had a [`Mass`] component.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub(crate) struct NonPickupMass(pub Mass);

/// Marker component for props that are held by an [`AvianPickupActor`].
#[derive(Debug, Clone, Copy, PartialEq, Component, Hash, Default, Reflect)]
#[reflect(Debug, Component, Default, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct HeldProp;
