use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupConfig>()
        .init_resource::<AvianPickupConfig>();
}

/// Configuration for the Avian Pickup plugin. Can be overridden by inserting it
/// after adding the plugin.
///
/// # Example
///
/// ```no_run
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
/// # use avian3d::prelude::*;
///
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         PhysicsPlugins::default(),
///         AvianPickupPlugin::default(),
///     ))
///     .insert_resource(AvianPickupConfig {
///         trace_length: 500.0,
///         ..default()
///     });
/// ```
#[derive(Debug, Clone, Resource, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupConfig {
    /// The spatial query filter to use when looking for objects to pick up.
    /// Default: All entities
    ///
    /// In addition, the following entities are always excluded:
    /// - The entity holding
    ///   [`AvianPickupCamera`](crate::prelude::AvianPickupCamera)
    /// - All colliders that do not belong to a [`RigidBody::Dynamic`]
    pub spatial_query_filter: SpatialQueryFilter,
    /// How far an object can be pulled from. Default: 250.0
    ///
    /// Corresponds to Source's [`physcannon_tracelength`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_tracelength).
    pub trace_length: f32,
    /// Changes how wide the pickup range is, lower numbers are wider. This is a
    /// dot product value. Default: 0.97
    ///
    /// Corresponds to Source's [`physcannon_cone`](https://developer.valvesoftware.com/wiki/Weapon_physcannon#physcannon_cone).
    pub cone: f32,
}

impl Default for AvianPickupConfig {
    fn default() -> Self {
        Self {
            spatial_query_filter: default(),
            trace_length: 250.0,
            cone: 0.97,
        }
    }
}
