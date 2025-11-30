use crate::prelude::*;

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
    Throw(Entity),
    /// Insert [`Dropping`] component and clear others
    Drop {
        /// The prop to drop
        prop: Entity,
        /// Whether the drop was forced to be dropped by
        /// being too far away from its target location.
        forced: bool,
    },
    /// Insert [`Pulling`] component and clear others
    Pull,
    /// Insert [`Holding`] component and clear others
    Hold(Entity),
}

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Throwing(pub(crate) Entity);

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Dropping {
    pub(crate) prop: Entity,
    pub(crate) forced: bool,
}

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Pulling;

/// Component inserted on an actor when they are holding a prop.
#[derive(Debug, Clone, Copy, Component)]
pub struct Holding(pub Entity);

/// Sets or clears the [`Verb`] of an actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SetVerb(pub(crate) Option<Verb>);

impl SetVerb {
    pub(crate) fn new(verb: impl Into<Option<Verb>>) -> Self {
        Self(verb.into())
    }
}

impl EntityCommand for SetVerb {
    fn apply(self, entity_world: EntityWorldMut) {
        let actor = entity_world.id();
        entity_world
            .into_world_mut()
            .run_system_cached_with(set_verb, (actor, self.0))
            .unwrap();
    }
}

fn set_verb(
    In((actor, verb)): In<(Entity, Option<Verb>)>,
    mut commands: Commands,
    q_actor: Query<(Has<Throwing>, Has<Dropping>, Has<Pulling>, Has<Holding>)>,
) {
    let Ok((throwing, dropping, pulling, holding)) = q_actor.get(actor) else {
        error!("Actor entity was deleted or in an invalid state. Ignoring.");
        return;
    };
    let mut commands = commands.entity(actor);
    match verb {
        Some(Verb::Throw(prop)) => {
            if !throwing {
                commands.try_insert(Throwing(prop));
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
        Some(Verb::Drop { prop, forced }) => {
            if !dropping {
                commands.try_insert(Dropping { prop, forced });
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
                commands.try_insert(Pulling);
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
                commands.try_insert(Holding(prop));
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
