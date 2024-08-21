//! TODO

use avian3d::prelude::*;
use bevy::{prelude::*, utils::HashSet};

use crate::{
    interaction::{HoldError, ShadowParams},
    prelude::{AvianPickupActor, AvianPickupActorState, Cooldown},
    verb::{SetVerb, Verb},
};

pub(super) mod prelude {
    pub use super::{AvianPickupInput, AvianPickupInputKind};
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupInput>()
        .add_event::<AvianPickupInput>()
        .add_systems(PostUpdate, set_verbs_according_to_input);
}

/// Event for picking up and throwing objects.
/// Send this to tell Avian Pickup to do its thing.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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
    pub kind: AvianPickupInputKind,
}

/// The kind of input that the [`AvianPickupInput`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum AvianPickupInputKind {
    /// The left mouse button was just pressed this update.
    JustPressedL,
    /// The right mouse button was just pressed this update.
    JustPressedR,
    /// The right mouse button was pressed.
    PressedR,
}

fn set_verbs_according_to_input(
    mut r_input: EventReader<AvianPickupInput>,
    mut commands: Commands,
    q_actor: Query<
        (
            Entity,
            Option<&AvianPickupActorState>,
            Option<&Cooldown>,
            Has<Position>,
            Has<Rotation>,
            Has<ShadowParams>,
            Has<HoldError>,
        ),
        With<AvianPickupActor>,
    >,
) {
    let mut unhandled_actors: HashSet<_> = q_actor.iter().map(|(entity, ..)| entity).collect();
    'outer: for &event in r_input.read() {
        let kind = event.kind;
        let actor = event.actor;
        unhandled_actors.remove(&actor);
        let Ok((_entity, state, cooldown, has_position, has_rotation, has_shadow, has_error)) =
            q_actor.get(actor)
        else {
            error!(
                "`AvianPickupEvent` was triggered on an entity without `AvianPickupActor`. Ignoring."
            );
            continue;
        };

        // Doing these checks now so that later systems can just call `unwrap`
        let checks = [(has_position, "Position"), (has_rotation, "Rotation")];
        for (has_component, component_name) in checks.iter() {
            if !has_component {
                error!(
                    "`AvianPickupEvent` was triggered on an entity without `{component_name}`. Ignoring.\n\
                    To fix this, try making the `AvianPickupActor` a rigid body or a descendant of one.\n\
                    Note that this can be done without adding a collider to the actor if you don't need one."
                );
                continue 'outer;
            }
        }
        // Doing these checks now so that later systems can just call `unwrap`
        let checks = [(has_shadow, "ShadowParams"), (has_error, "HoldError")];
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

        let verb = match kind {
            AvianPickupInputKind::JustPressedL if cooldown.left.finished() => {
                if let AvianPickupActorState::Holding(prop) = state {
                    Some(Verb::Throw(Some(prop)))
                } else {
                    Some(Verb::Throw(None))
                }
            }
            AvianPickupInputKind::JustPressedL => None,
            AvianPickupInputKind::JustPressedR
                if matches!(state, AvianPickupActorState::Holding(..))
                    && cooldown.right.finished() =>
            {
                let AvianPickupActorState::Holding(prop) = state else {
                    unreachable!()
                };
                Some(Verb::Drop(prop))
            }
            AvianPickupInputKind::JustPressedR | AvianPickupInputKind::PressedR => {
                if matches!(
                    state,
                    AvianPickupActorState::Idle | AvianPickupActorState::Pulling(..)
                ) && cooldown.right.finished()
                {
                    Some(Verb::Pull)
                } else {
                    None
                }
            }
        };
        commands.entity(actor).add(SetVerb::new(verb));
    }
    for &actor in unhandled_actors.iter() {
        commands.entity(actor).add(SetVerb::new(None));
    }
}
