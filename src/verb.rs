use bevy::{
    ecs::system::{EntityCommand, RunSystemOnce},
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verb {
    Throw(Option<Entity>),
    Drop(Entity),
    Pull,
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
            if pulling {
                commands.remove::<Pulling>();
            }
            if holding {
                commands.remove::<Holding>();
            }
            if throwing {
                commands.remove::<Throwing>();
            }
            if dropping {
                commands.remove::<Dropping>();
            }
        }
    }
}
