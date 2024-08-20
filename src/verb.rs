use bevy::{
    ecs::system::{EntityCommand, RunSystemOnce},
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

/// This marks a state transition coming from either
/// an external [`AvianPickupInput`](crate::prelude::AvianPickupInput)
/// or an a state transition as dictated by the handling of the current state.
/// Not to be confused with
/// [`AvianPickupActorState`](crate::prelude::AvianPickupActorState),
/// as that one is the user-facing way of communicating what the current state
/// is.
///
/// This type itself is just an usher for the actual marker components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verb {
    /// Insert [`Throwing`] component and clear others
    Throw(Option<Entity>),
    /// Insert [`Dropping`] component and clear others
    Drop(Entity),
    /// Insert [`Pulling`] component and clear others
    Pull,
    /// Insert [`Holding`] component and clear others
    Hold(Entity),
}

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Throwing(pub(crate) Option<Entity>);

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Dropping(pub(crate) Entity);

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Pulling;

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Holding(pub(crate) Entity);

/// Sets or clears the [`Verb`] of an actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SetVerb(pub(crate) Option<Verb>);

impl SetVerb {
    pub(crate) fn new(verb: impl Into<Option<Verb>>) -> Self {
        Self(verb.into())
    }
}

impl EntityCommand for SetVerb {
    fn apply(self, actor: Entity, world: &mut World) {
        world.run_system_once_with((actor, self.0), set_verb);
    }
}

fn set_verb(
    In((actor, verb)): In<(Entity, Option<Verb>)>,
    mut commands: Commands,
    q_actor: Query<(Has<Throwing>, Has<Dropping>, Has<Pulling>, Has<Holding>)>,
) {
    // Safety: we are only querying optional components.
    let (throwing, dropping, pulling, holding) = q_actor.get(actor).unwrap();
    let mut commands = commands.entity(actor);
    match verb {
        Some(Verb::Throw(prop)) => {
            if !throwing {
                commands.insert(Throwing(prop));
            }
            if dropping {
                commands.remove::<Dropping>();
            }
            if pulling {
                commands.remove::<Pulling>();
            }
            if holding {
                commands.remove::<Holding>();
            }
        }
        Some(Verb::Drop(prop)) => {
            if !dropping {
                commands.insert(Dropping(prop));
            }
            if throwing {
                commands.remove::<Throwing>();
            }
            if pulling {
                commands.remove::<Pulling>();
            }
            if holding {
                commands.remove::<Holding>();
            }
        }
        Some(Verb::Pull) => {
            if !pulling {
                commands.insert(Pulling);
            }
            if throwing {
                commands.remove::<Throwing>();
            }
            if dropping {
                commands.remove::<Dropping>();
            }
            if holding {
                commands.remove::<Holding>();
            }
        }
        Some(Verb::Hold(prop)) => {
            if !holding {
                commands.insert(Holding(prop));
            }
            if throwing {
                commands.remove::<Throwing>();
            }
            if dropping {
                commands.remove::<Dropping>();
            }
            if pulling {
                commands.remove::<Pulling>();
            }
        }
        None => {
            // `Throwing` and `Dropping` clean up after themselves.
            // `Holding` should never be just removed, but only replaced by `Throwing` or
            // `Dropping`. `Pulling` in the meantime should only be present
            // while we are holding a button -> we can remove it here.
            if pulling {
                commands.remove::<Pulling>();
            }
        }
    }
}
