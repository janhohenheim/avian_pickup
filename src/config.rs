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
/// ```
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
#[derive(Debug, Clone, Copy, Resource, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupConfig {
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
            trace_length: 250.0,
            cone: 0.97,
        }
    }
}
