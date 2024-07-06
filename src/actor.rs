use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{math::METERS_PER_HAMMER_UNIT, spatial_query_filter::prepare_spatial_query_filter};

pub(super) mod prelude {
    pub use super::{AvianPickupActor, AvianPickupActorState};
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupActor>()
        .observe(add_state_to_actor.pipe(prepare_spatial_query_filter))
        .observe(on_collider_constructor_hierarchy_finished.pipe(prepare_spatial_query_filter));
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
#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Debug, Component, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupActor {
    /// The spatial query filter to use when looking for objects to pick up.
    /// Default: All entities
    ///
    /// For your convenience, the following entities will be added to the filter
    /// automatically when the component is added:
    /// - The [`AvianPickupActor`], if it is a [`Collider`]
    /// - All descendants of the [`AvianPickupActor`] that are [`Collider`]s
    ///
    /// If the [`AvianPickupActor`] is a [`ColliderConstructorHierarchy`], the
    /// entities will be added when the colliders have actually been
    /// constructed.
    ///
    /// In addition, all colliders that do not belong to a
    /// [`RigidBody::Dynamic`] will implicitly be filtered out.
    pub spatial_query_filter: SpatialQueryFilter,
    /// How far an object can be pulled from in meters. Default: 6.35 m
    ///
    /// Corresponds to Source's [`physcannon_tracelength`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_tracelength).
    pub trace_length: f32,
    /// Changes how wide the pickup range is, lower numbers are wider. This is a
    /// dot product value. Default: 0.97
    ///
    /// Corresponds to Source's [`physcannon_cone`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_cone).
    pub cone: f32,
    /// The maximum mass in kg an object can have to be picked up. Default: 35.0
    /// kg
    ///
    /// Corresponds to Source's [`physcannon_maxmass`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_maxmass).
    pub max_mass: f32,
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
    /// The actor is throwing an object.
    Throwing(Entity),
}

impl Default for AvianPickupActor {
    fn default() -> Self {
        Self {
            spatial_query_filter: default(),
            trace_length: 250.0 * METERS_PER_HAMMER_UNIT,
            cone: 0.97,
            max_mass: 35.0,
        }
    }
}

fn add_state_to_actor(trigger: Trigger<OnAdd, AvianPickupActor>, mut commands: Commands) -> Entity {
    let entity = trigger.entity();
    commands
        .entity(entity)
        .insert(AvianPickupActorState::default());
    entity
}

fn on_collider_constructor_hierarchy_finished(
    trigger: Trigger<OnRemove, ColliderConstructorHierarchy>,
) -> Entity {
    trigger.entity()
}
