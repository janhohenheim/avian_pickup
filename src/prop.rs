//! TODO

use avian3d::math::Scalar;
use bevy::prelude::*;

use crate::prelude::AvianPickupActor;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(
        PreferredPickupRotation,
        ClampPickupPitchOverride,
        PreferredPickupDistanceOverride,
        PickupMassOverride,
    )>();
}

pub(super) mod prelude {
    pub use super::{
        ClampPickupPitchOverride,
        PickupMassOverride,
        PreferredPickupDistanceOverride,
        PreferredPickupRotation,
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
#[derive(Debug, Clone, Copy, PartialEq, Component, Default, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PreferredPickupRotation(pub Quat);

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub(crate) struct PrePickupRotation(pub Quat);

/// Insert this on a prop to override
/// [`AvianPickupActor::clamp_pickup_pitch`](crate::prelude::AvianPickupActor::clamp_pickup_pitch).
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct ClampPickupPitchOverride {
    /// The minimum pitch the held object can have in radians while following
    /// the actor's pitch.
    pub min: f32,
    /// The maximum pitch the held object can have in radians while following
    /// the actor's pitch.
    pub max: f32,
}
impl Default for ClampPickupPitchOverride {
    fn default() -> Self {
        let default_actor = AvianPickupActor::default();
        Self {
            min: default_actor.clamp_pickup_pitch.0,
            max: default_actor.clamp_pickup_pitch.1,
        }
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActor::preferred_pickup_distance`](crate::prelude::AvianPickupActor::preferred_pickup_distance).
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
        Self(AvianPickupActor::default().preferred_pickup_distance)
    }
}

/// Insert this on a prop to override
/// [`AvianPickupActor::pickup_mass`](crate::prelude::AvianPickupActor::pickup_mass).
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
        Self(AvianPickupActor::default().pickup_mass)
    }
}

/// The cached mass that an object had before it was picked up
/// that will be restored again when it is dropped.
/// In other words, this is the mass before and after the pickup.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub(crate) struct NonPickupMass(pub Scalar);
