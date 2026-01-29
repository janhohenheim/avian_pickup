//! Module for the actor that can pick up objects.

use std::ops::RangeInclusive;

use avian3d::{math::Scalar, prelude::*};

use crate::{
    interaction::{HoldError, ShadowParams},
    prelude::*,
};

pub(super) mod prelude {
    pub use super::{
        AvianPickupActor, AvianPickupActorHoldConfig, AvianPickupActorPullConfig,
        AvianPickupActorState, AvianPickupActorThrowConfig,
    };
}

pub(super) fn plugin(_app: &mut App) {}

/// Tag component for an actor that is able to pick up object.
/// For a first-person game, add this to the camera entity that is under the
/// player control.
///
/// Requires the entity to also hold a [`Transform`] and [`GlobalTransform`].
///
/// # Example
///
/// ```
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// fn setup_camera(mut commands: Commands) {
///     commands.spawn((
///         Name::new("Player Camera"),
///         Camera3d::default(),
///         AvianPickupActor::default(),
///     ));
/// }
/// ```
#[derive(Debug, Component, Clone, PartialEq, Reflect)]
#[reflect(Debug, Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
#[require(AvianPickupActorState, Cooldown, HoldError, ShadowParams)]
pub struct AvianPickupActor {
    /// The spatial query filter to use when looking for objects to pick up.\
    /// Note that no matter what this filter says, only entities with a
    /// [`RigidBody::Dynamic`] will be considered in the first place.\
    ///
    /// Default: Include all entities
    pub prop_filter: SpatialQueryFilter,
    /// The spatial query filter to use when looking for terrain that will block
    /// picking up a prop behind it.\
    /// Default: Include all entities
    pub obstacle_filter: SpatialQueryFilter,
    /// The spatial query filter to use when looking colliders belonging to this
    /// actor.\
    /// This is used to filter out colliders that should not be
    /// taken into account when calculating the actor's total rigid body
    /// extent.\
    /// Default: Include all entities
    pub actor_filter: SpatialQueryFilter,
    /// How far away an object can be interacted with.\
    /// Default: 5.0 m
    ///
    /// Corresponds to Source's [`physcannon_tracelength`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_tracelength) * 4.0.
    pub interaction_distance: Scalar,
    /// Changes how wide the pickup range is. Lower numbers are wider.
    /// This is the dot product of the direction the player is looking and the
    /// direction to the object.\
    /// Default: 0.97
    ///
    /// Corresponds to Source's [`physcannon_cone`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_cone).
    pub interaction_cone: f32,
    /// Configuration that is only used when pulling props to the actor.
    pub pull: AvianPickupActorPullConfig,
    /// Configuration that is only used while holding props.
    pub hold: AvianPickupActorHoldConfig,
    /// Configuration that is only used when throwing props.
    pub throw: AvianPickupActorThrowConfig,
}

/// Configuration that is only used when pulling props to the actor.
/// Used in [`AvianPickupActor::pull`].
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupActorPullConfig {
    /// How much impulse to be used when pulling objects to the player.
    /// This is applied every 0.1 seconds.\
    /// Default: 100.0 Ns
    ///
    /// Corresponds to Source's [`physcannon_pullforce`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_pullforce).
    pub impulse: Scalar,
    /// The maximum mass in kg an object can have to be pulled or picked up.\
    /// Default: 35.0 kg
    ///
    /// Corresponds to Source's [`physcannon_maxmass`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_maxmass).
    pub max_prop_mass: Scalar,
}

impl Default for AvianPickupActorPullConfig {
    fn default() -> Self {
        Self {
            impulse: 100.0,
            max_prop_mass: 35.0,
        }
    }
}

/// Configuration that is only used while holding props.
/// Used in [`AvianPickupActor::hold`].
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupActorHoldConfig {
    /// The maximum distance between the player and the object when it can be
    /// picked up.\
    /// "distance" in this context is the distance between the edge of the prop
    /// and the origin of the actor.\
    /// Default: 3.0 m
    ///
    /// Corresponds to Source's [`physcannon_tracelength`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_tracelength)
    pub distance_to_allow_holding: Scalar,
    /// The minimum distance an object must be from the player when picked up.
    /// "distance" in this context is the distance between the edge of the prop
    /// and the origin of the actor.\
    /// Usually, the prop will try to stay at
    /// [`preferred_distance`](Self::preferred_distance),
    /// but will fall back to this when there is terrain in the way as
    /// determined by
    /// [`AvianPickupActor::obstacle_filter`](AvianPickupActor::obstacle_filter).
    /// \ If the actor is a rigid body, the distance used in that case is
    /// `max(collider_radius, min_distance)`.\
    /// Default: 0.2 m
    pub min_distance: Scalar,
    /// A number >= 0 that indicates how much exponential easing will be applied
    /// to the held prop's velocity when the actor is moving.\
    /// A value of 0 means no smoothing, i.e. the prop perfectly follows the
    /// actor's position.\
    /// Default: 1.0
    pub linear_velocity_easing: Scalar,
    /// A number >= 0 that indicates how much exponential easing will be applied
    /// to the held prop's angular velocity when the actor is rotating.\
    /// A value of 0 means no smoothing, i.e. the prop perfectly follows the
    /// actor's point of view.\
    /// Default: 1.6
    pub angular_velocity_easing: Scalar,
    /// The minimum and maximum pitch the held prop can have in radians while
    /// following the actor's pitch.\
    /// Can be overridden by adding a
    /// [`PitchRangeOverride`] to the prop.\
    /// Default: (-75.0).to_radians() to 75.0.to_radians()
    pub pitch_range: RangeInclusive<f32>,
    /// The distance in meters between the player and the object when
    /// picked up and there is no obstacle in the way.\
    /// "distance" in this context is the distance between the edge of the prop
    /// and the origin of the actor.\
    /// Can be overridden by adding a
    /// [`PreferredPickupDistanceOverride`]
    /// to the prop.\
    /// Default: 0.6 m
    pub preferred_distance: Scalar,
    /// The mass in kg of the object when picked up.
    /// This mechanism is needed because the held object's velocity is
    /// set directly, independent of its mass. This means that heavy
    /// objects could potentially generate *a lot* of force when colliding
    /// with other objects.
    /// The prop's original mass will be restored when the prop is no longer
    /// being held\
    /// Can be overridden by adding a
    /// [`PickupMassOverride`] to the prop.\
    /// Default: 1 kg
    pub temporary_prop_mass: Scalar,
}

impl Default for AvianPickupActorHoldConfig {
    fn default() -> Self {
        Self {
            distance_to_allow_holding: 3.0,
            min_distance: 0.2,
            linear_velocity_easing: 1.0,
            angular_velocity_easing: 1.6,
            pitch_range: (-75.0_f32).to_radians()..=75.0_f32.to_radians(),
            preferred_distance: 0.6,
            temporary_prop_mass: 1.0,
        }
    }
}

/// Configuration that is only used when throwing props.
/// Used in [`AvianPickupActor::throw`].
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupActorThrowConfig {
    /// Objects with less than this mass will be thrown with
    /// `linear_speed_range.end()`.\
    /// Objects with more than this mass will be thrown with
    /// less speed, down to objects at
    /// [`AvianPickupActorPullConfig::max_prop_mass`], which will be thrown
    /// with `linear_speed_range.start()`.\
    /// Default: 20.0 kg
    pub cutoff_mass_for_slowdown: Scalar,
    /// The range of linear speeds in m/s that the object can be thrown with.\
    /// The lower bound is used for very heavy objects, the upper bound for \
    /// very light objects.\
    /// Can be overridden by adding a
    /// [`ThrownLinearSpeedOverride`] to the prop.\
    /// Default: 0.0 m/s to 5.0 m/s
    pub linear_speed_range: RangeInclusive<Scalar>,
    /// The range of angular speeds in rad/s that the object can be thrown with.
    /// When throwing, a random value in this range will be chosen.\
    /// Can be overridden by adding a
    /// [`ThrownAngularSpeedOverride`] to the prop.\
    /// Default: 0.0 rad/s to 1.0 rad/s
    pub angular_speed_range: RangeInclusive<Scalar>,
}

impl Default for AvianPickupActorThrowConfig {
    fn default() -> Self {
        Self {
            cutoff_mass_for_slowdown: 20.0,
            linear_speed_range: 0.0..=5.0,
            angular_speed_range: 0.0..=1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component, Default, Reflect)]
#[reflect(Debug, Component, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
/// The state of an [`AvianPickupActor`]. This component is automatically added
/// to the entity holding the [`AvianPickupActor`], do not add or remove it.\
/// If you need to react to a prop being thrown or dropped, listen to
/// [`PropThrown`] and [`PropDropped`]
pub enum AvianPickupActorState {
    /// The actor is not doing anything.
    #[default]
    Idle,
    /// The actor is trying to pick up an object.
    /// The object is still too far away to be picked up,
    /// so we're pulling it closer.
    Pulling(Entity),
    /// The actor is holding an object.
    Holding(Entity),
}

impl AvianPickupActorState {
    /// Returns `true` if the actor is [`AvianPickupActorState::Idle`].
    pub fn is_idle(self) -> bool {
        matches!(self, AvianPickupActorState::Idle)
    }

    /// Returns `true` if the actor is [`AvianPickupActorState::Pulling`].
    pub fn is_pulling(self) -> bool {
        matches!(self, AvianPickupActorState::Pulling { .. })
    }

    /// Returns `true` if the actor is [`AvianPickupActorState::Holding`].
    pub fn is_holding(self) -> bool {
        matches!(self, AvianPickupActorState::Holding { .. })
    }
}

impl Default for AvianPickupActor {
    fn default() -> Self {
        Self {
            prop_filter: default(),
            obstacle_filter: default(),
            actor_filter: default(),
            interaction_distance: 5.0,
            interaction_cone: 0.97,
            pull: default(),
            hold: default(),
            throw: default(),
        }
    }
}
