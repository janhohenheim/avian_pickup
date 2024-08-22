//! TODO

use avian3d::{math::Scalar, prelude::*};
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

use crate::{
    interaction::{HoldError, ShadowParams},
    prelude::Cooldown,
};

pub(super) mod prelude {
    pub use super::{AvianPickupActor, AvianPickupActorState};
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(AvianPickupActor, AvianPickupActorState)>();
}

/// Tag component for an actor that is able to pick up object.
/// For a first-person game, add this to the camera entity that is under the
/// player control.
///
/// Requires the entity to also hold [`TransformBundle`].
///
/// # Example
/// ```
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// fn setup_camera(mut commands: Commands) {
///     commands.spawn((
///         Name::new("Player Camera"),
///         Camera3dBundle::default(),
///         AvianPickupActor::default(),
///     ));
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(Debug, Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
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
    /// How far an object can be pulled from in meters. Default: 3 m
    ///
    /// Corresponds to Source's [`physcannon_tracelength`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_tracelength).
    pub trace_length: Scalar,
    /// Changes how wide the pickup range is, lower numbers are wider. This is a
    /// dot product value. Default: 0.97
    ///
    /// Corresponds to Source's [`physcannon_cone`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_cone).
    pub cone: f32,
    /// The maximum mass in kg an object can have to be picked up. Default: 35.0
    /// kg
    ///
    /// Corresponds to Source's [`physcannon_maxmass`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_maxmass).
    pub max_mass: Scalar,
    /// How much force to be used when pulling objects to the player.
    /// Default: 100.0 N
    ///
    /// Corresponds to Source's [`physcannon_pullforce`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_pullforce).
    pub pull_force: Scalar,
    /// The minimum distance an object must be from the player when picked up.
    /// Usually, the prop will try to stay at
    /// [`PreferredPickupDistance`](crate::prop::PreferredPickupDistance),
    /// but will fall back to this when there is terrain in the way as
    /// determined by [`terrain_filter`](Self::terrain_filter).\
    /// If the actor is a rigid body, the distance used in that case is
    /// `max(collider_radius, min_distance)`.\
    /// Default: 0.5 m
    pub min_distance: Scalar,
    /// A number >= 0 that indicates how much exponential easing will be applied
    /// to the held prop's velocity when the actor is moving.\
    /// A value of 0 means no smoothing, i.e. the prop perfectly follows the
    /// actor's position.\
    /// Default: 0.0
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
    /// [`ClampPickupPitchOverride`](crate::prop::ClampPickupPitchOverride)
    /// to the prop.\
    /// Default: (-75.0).to_radians() to 75.0.to_radians()
    pub clamp_pickup_pitch: (f32, f32),
    /// The distance in meters between the player and the object's OBBs when
    /// picked up and there is no obstacle in the way.\
    /// Can be overridden by adding a
    /// [`PreferredPickupDistanceOverride`](crate::prop::PreferredPickupDistanceOverride)
    /// to the prop.\
    /// Default: 1.5 m
    pub preferred_pickup_distance: Scalar,
    /// The mass in kg of the object when picked up.
    /// This mechanism is needed because the held object's velocity is
    /// set directly, independent of its mass. This means that heavy
    /// objects could potentially generate *a lot* of force when colliding
    /// with other objects.
    /// The prop's original mass will be restored when the prop is no longer
    /// being held\
    /// Can be overridden by adding a
    /// [`PickupMassOverride`](crate::prop::PickupMassOverride) to the prop.\
    /// Default: 1 kg
    pub pickup_mass: Scalar,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component, Default, Reflect)]
#[reflect(Debug, Component, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
/// The state of an [`AvianPickupActor`]. This component is automatically added
/// to the entity holding the [`AvianPickupActor`], do not add or remove it.
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

impl Default for AvianPickupActor {
    fn default() -> Self {
        Self {
            prop_filter: default(),
            obstacle_filter: default(),
            actor_filter: default(),
            trace_length: 3.,
            cone: 0.97,
            max_mass: 35.0,
            pull_force: 100.0,
            min_distance: 0.5,
            linear_velocity_easing: 1.0,
            angular_velocity_easing: 1.6,
            clamp_pickup_pitch: (-75.0_f32.to_radians(), 75.0_f32.to_radians()),
            preferred_pickup_distance: 1.5,
            pickup_mass: 1.0,
        }
    }
}

impl Component for AvianPickupActor {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, targeted_entity, _component_id| {
            let mut commands = world.commands();
            commands.entity(targeted_entity).insert((
                AvianPickupActorState::default(),
                Cooldown::default(),
                HoldError::default(),
                ShadowParams::default(),
            ));
        });
    }
}
