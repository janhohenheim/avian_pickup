#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use avian3d::prelude::*;
use bevy::prelude::*;

mod camera;

/// Everything you need to use the Avian Pickup plugin.
pub mod prelude {
    pub use crate::{camera::AvianPickupCamera, AvianPickupPlugin};
}

#[derive(Default)]
#[non_exhaustive]
/// The Avian Pickup plugin. Add this after the Physics plugins to enable pickup functionality.
/// Uses the same [`Schedule`]` as Avian.
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
        let physics_schedule = app.get_schedule_mut(PhysicsSchedule).expect(
            "Failed to build `AvianPickupPlugin`:\
                Avian's `PhysicsSchedule` was not found. Make sure to add Avian's plugins *before* `AvianPickupPlugin`.\
                This usually done by adding `PhysicsPlugins` to your `App`.",
        );

        physics_schedule.configure_sets(AvianPickupSystem::First.in_set(PhysicsStepSet::First));
        app.add_plugins(camera::plugin);
    }
}

/// Set enum for the systems relating to accessibility.
/// Use this to order your systems relative to the ones used by Avian Pickup.
/// This is run in Avian's `PhysicsStepSet::First`.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AvianPickupSystem {
    /// Runs at the start of the [`AvianPickupSystem`]. Empty by default.
    First,
}
