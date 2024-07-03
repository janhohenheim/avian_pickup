#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use bevy::prelude::*;

/// Everything you need to use the Avian Pickup plugin.
pub mod prelude {
    //pub use crate::{AvianPickupCamera, AvianPickupPlugin};
}

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
/// The Avian Pickup plugin. Add this after the Physics plugins to enable pickup functionality.
///
/// # Example
///
/// ```no_run
/// //# use avian3d::prelude::*;
/// //# use avian_pickup::prelude::*;
/// //# use bevy::prelude::*;
///
/// //App::new().add_plugins((DefaultPlugins, PhysicsPlugins::default(), AvianPickupPlugin::default()));
/// ```
pub struct AvianPickupPlugin;

impl Plugin for AvianPickupPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Clone, Copy, Hash, Component, Default, PartialEq, Eq, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", reflect(Serialize, Deserialize))]
#[reflect(Debug, Component, Default, Hash, PartialEq)]
/// Tag component for the camera that will be used for picking up objects.
/// Place this on the camera entity that is under the player control.
///
/// # Example
/// ```
/// use avian_pickup::prelude::*;
/// ```
pub struct AvianPickupCamera;
