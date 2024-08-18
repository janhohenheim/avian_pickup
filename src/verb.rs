use bevy::{
    ecs::system::{EntityCommand, EntityCommands, RunSystemOnce},
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verb {
    Throw,
    Drop,
    Pull,
    Hold(Entity),
}

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Throwing;

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Dropping;

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Pulling;

#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Holding(pub(crate) Entity);

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
    match verb {
        Some(Verb::Throw) => {
            if throwing {
                return;
            }
            commands
                .entity(actor)
                .insert(Throwing)
                .remove::<(Dropping, Pulling, Holding)>();
        }
        Some(Verb::Drop) => {
            if dropping {
                return;
            }
            commands
                .entity(actor)
                .insert(Dropping)
                .remove::<(Throwing, Pulling, Holding)>();
        }
        Some(Verb::Pull) => {
            if pulling {
                return;
            }
            commands
                .entity(actor)
                .insert(Pulling)
                .remove::<(Throwing, Dropping, Holding)>();
        }
        Some(Verb::Hold(prop)) => {
            if holding {
                return;
            }
            commands
                .entity(actor)
                .insert(Holding(prop))
                .remove::<(Throwing, Dropping, Pulling)>();
        }
        None => {
            commands
                .entity(actor)
                .remove::<(Throwing, Dropping, Pulling, Holding)>();
        }
    }
}
