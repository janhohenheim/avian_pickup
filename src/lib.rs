#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use bevy::prelude::*;

/// Everything you need to use the Avian Pickup plugin.
pub mod prelude {
    pub use crate::AvianPickupPlugin;
}

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
/// The Avian Pickup plugin. Add this after the Physics plugins to enable pickup functionality.
///
/// # Example
///
/// ```no_run
/// # use avian3d::prelude::*;
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// App::new().add_plugins((DefaultPlugins, PhysicsPlugins::default(), AvianPickupPlugin::default()));
/// ```
pub struct AvianPickupPlugin;

impl Plugin for AvianPickupPlugin {
    fn build(&self, _app: &mut App) {}
}
