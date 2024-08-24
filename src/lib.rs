#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use avian3d::prelude::*;
use bevy::prelude::*;

pub mod actor;
mod cooldown;
pub mod input;
mod interaction;
mod math;
pub mod output;
pub mod prop;
mod rng;
mod verb;

pub use rng::*;
/// Everything you need to get started with Avian Pickup.
pub mod prelude {
    pub(crate) use avian3d::prelude::*;
    pub(crate) use bevy::prelude::*;

    pub(crate) use super::{
        cooldown::prelude::*,
        prop::{prelude::*, NonPickupMass},
        HandleVerbSystem,
    };
    pub use crate::{
        actor::prelude::*,
        input::prelude::*,
        output::prelude::*,
        AvianPickupPlugin,
        AvianPickupSystem,
    };
}

/// The Avian Pickup plugin. Add this after the Avian Physics plugins to enable
/// pickup functionality. Uses the same [`Schedule`]` as Avian.
///
/// # Example
///
/// ```no_run
/// # use avian3d::prelude::*;
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// App::new().add_plugins((
///     DefaultPlugins,
///     PhysicsPlugins::default(),
///     AvianPickupPlugin::default(),
/// ));
/// ```
#[derive(Default)]
#[non_exhaustive]
pub struct AvianPickupPlugin;

impl Plugin for AvianPickupPlugin {
    fn build(&self, app: &mut App) {
        let physics_schedule = app.get_schedule(PhysicsSchedule);
        if physics_schedule.is_none() {
            panic!(
                "Failed to build `AvianPickupPlugin`:\
                Avian's `PhysicsSchedule` was not found. Make sure to add Avian's plugins *before* `AvianPickupPlugin`.\
                This usually done by adding `PhysicsPlugins` to your `App`."
            )
        }

        app.configure_sets(
            PhysicsSchedule,
            (
                AvianPickupSystem::First,
                AvianPickupSystem::HandleVerb,
                AvianPickupSystem::ResetIdle,
                AvianPickupSystem::TickTimers,
                AvianPickupSystem::Last,
            )
                .chain()
                .in_set(PhysicsStepSet::First),
        )
        .configure_sets(
            PhysicsSchedule,
            (
                HandleVerbSystem::Pull,
                HandleVerbSystem::Hold,
                HandleVerbSystem::Drop,
                HandleVerbSystem::Throw,
            )
                .chain()
                .in_set(AvianPickupSystem::HandleVerb),
        );

        app.add_plugins((
            input::plugin,
            output::plugin,
            actor::plugin,
            interaction::plugin,
            cooldown::plugin,
            prop::plugin,
            verb::plugin,
            rng::plugin,
        ));
    }
}

/// Set enum for the systems added by [`AvianPickupPlugin`].
/// Use this to order your systems relative to the ones used by Avian Pickup.
/// This is run in Avian's `PhysicsStepSet::First`.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AvianPickupSystem {
    /// Runs at the start of the [`AvianPickupSystem`]. Empty by default.
    First,
    /// Adds forces to an object held by
    /// [`AvianPickupActorState::Holding`](crate::prelude::AvianPickupActorState::Holding)
    /// in order to keep it in place in front of the
    /// [`AvianPickupActor`](crate::prelude::AvianPickupActor).
    HandleVerb,
    /// Resets the
    /// [`AvianPickupActorState`](crate::prelude::AvianPickupActorState) to
    /// [`AvianPickupActorState::Idle`](crate::prelude::AvianPickupActorState::Idle)
    /// if needed
    ResetIdle,
    /// Advances internal cooldown timers.
    TickTimers,
    /// Runs at the end of the [`AvianPickupSystem`]. Empty by default.
    Last,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum HandleVerbSystem {
    Pull,
    Hold,
    Drop,
    Throw,
}
