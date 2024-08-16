use avian3d::math::Scalar;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(PreferredPickupRotation, PickupMass)>();
}

/// Insert this on an object to set its rotation when picked up.
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

/// Insert this on an object to set its distance from the
/// [`AvianPickupActor`](crate::prelude::AvianPickupActor) when picked up.
///
/// If an object has no `PreferredPickupDistance`, it will be held at 1.5 meters
/// from the player.
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, PartialEq, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PreferredPickupDistance(pub Scalar);

impl Default for PreferredPickupDistance {
    fn default() -> Self {
        Self(1.5)
    }
}

/// Insert this on an object to set its mass in kg when picked up.
/// If missing, the object will be held with a mass of 1 kg.
///
/// This mechanism is needed because the held object's velocity is
/// set directly, independent of its mass. This means that heavy
/// objects could potentially generate *a lot* of force when colliding
/// with other objects.
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PickupMass(pub Scalar);

impl Default for PickupMass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub(crate) struct NonPickupMass(pub Scalar);
