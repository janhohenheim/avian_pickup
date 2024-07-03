#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use avian3d::prelude::PhysicsSchedule;
use bevy::prelude::*;

mod camera;

/// Everything you need to use the Avian Pickup plugin.
pub mod prelude {
    pub use crate::{camera::AvianPickupCamera, AvianPickupPlugin};
}

#[derive(Debug)]
#[non_exhaustive]
/// The Avian Pickup plugin. Add this after the Physics plugins to enable pickup functionality.
///
/// # Example
///
/// ```
/// # use avian3d::prelude::*;
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// App::new().add_plugins((DefaultPlugins, PhysicsPlugins::default(), AvianPickupPlugin::default()));
/// ```
pub struct AvianPickupPlugin;

impl Plugin for AvianPickupPlugin {
    fn build(&self, app: &mut App) {
        // Doing an `expect` here so that subplugins can just `unwrap`.
        let _physics_schedule = app.get_schedule_mut(PhysicsSchedule).expect(
            "Failed to build `AvianPickupPlugin`:\
                Avian's `PhysicsSchedule` was not found. Make sure to add Avian's plugins *before* `AvianPickupPlugin`.\
                This usually done by adding `PhysicsPlugins` to your `App`.",
        );

        app.add_plugins(camera::plugin);
    }
}
