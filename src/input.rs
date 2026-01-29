//! Module for the types that represent input events for Avian Pickup.

use bevy_platform::collections::HashSet;

use crate::{
    interaction::{HoldError, ShadowParams},
    prelude::*,
    verb::{SetVerb, Verb},
};

pub(super) mod prelude {
    pub use super::{AvianPickupAction, AvianPickupInput};
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<AvianPickupInput>()
        .add_systems(PostUpdate, set_verbs_according_to_input);
}

/// Message for picking up and throwing objects.
/// Send this to tell Avian Pickup to do its thing.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupInput {
    /// The entity of the [`AvianPickupActor`] that the event is related to.
    pub actor: Entity,
    /// The kind of input that the event represents.
    pub action: AvianPickupAction,
}

/// The kind of input that the [`AvianPickupInput`] represents.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum AvianPickupAction {
    /// The left mouse button was just pressed this update.
    Throw,
    /// The right mouse button was just pressed this update.
    Drop,
    /// The right mouse button was pressed.
    Pull,
}

impl AvianPickupAction {
    pub(crate) fn iter() -> impl Iterator<Item = Self> {
        [Self::Throw, Self::Drop, Self::Pull].iter().copied()
    }
}

fn set_verbs_according_to_input(
    mut r_input: MessageReader<AvianPickupInput>,
    mut commands: Commands,
    q_actor: Query<
        (
            Entity,
            Option<&AvianPickupActorState>,
            Option<&Cooldown>,
            Has<GlobalTransform>,
            Has<ShadowParams>,
            Has<HoldError>,
        ),
        With<AvianPickupActor>,
    >,
) {
    let mut unhandled_actors: HashSet<_> = q_actor.iter().map(|(entity, ..)| entity).collect();
    'outer: for &event in r_input.read() {
        let action = event.action;
        let actor = event.actor;
        unhandled_actors.remove(&actor);
        let Ok((_entity, state, cooldown, has_global_transform, has_shadow, has_error)) =
            q_actor.get(actor)
        else {
            error!(
                "`AvianPickupEvent` was triggered on an entity without `AvianPickupActor`. Ignoring."
            );
            continue;
        };

        // Doing these checks now so that we can report issues early.
        let checks = [
            (has_global_transform, "GlobalTransform"),
            (has_shadow, "ShadowParams"),
            (has_error, "HoldError"),
        ];
        for (has_component, component_name) in checks.iter() {
            if !has_component {
                error!(
                    "`AvianPickupEvent` was triggered on an entity without `{component_name}`. Ignoring."
                );
                continue 'outer;
            }
        }

        let Some(&state) = state else {
            error!(
                "`AvianPickupEvent` was triggered on an entity without `AvianPickupActorState`. Ignoring."
            );
            continue;
        };

        let Some(cooldown) = cooldown else {
            error!("`AvianPickupEvent` was triggered on an entity without `Cooldown`. Ignoring.");
            continue;
        };

        let verb = match action {
            AvianPickupAction::Throw
                if cooldown.is_finished(AvianPickupAction::Throw)
                    && matches!(state, AvianPickupActorState::Holding(..)) =>
            {
                let AvianPickupActorState::Holding(prop) = state else {
                    unreachable!()
                };
                Some(Verb::Throw(prop))
            }
            AvianPickupAction::Drop
                if matches!(state, AvianPickupActorState::Holding(..))
                    && cooldown.is_finished(AvianPickupAction::Drop) =>
            {
                let AvianPickupActorState::Holding(prop) = state else {
                    unreachable!()
                };
                Some(Verb::Drop {
                    prop,
                    forced: false,
                })
            }
            AvianPickupAction::Pull
                if matches!(
                    state,
                    AvianPickupActorState::Idle | AvianPickupActorState::Pulling(..)
                ) && cooldown.is_finished(AvianPickupAction::Pull) =>
            {
                Some(Verb::Pull)
            }
            _ => None,
        };
        commands.entity(actor).queue(SetVerb::new(verb));
    }
    for &actor in unhandled_actors.iter() {
        commands.entity(actor).queue(SetVerb::new(None));
    }
}
