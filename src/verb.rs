use bevy::{
    ecs::system::{EntityCommand, RunSystemOnce},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_add_dropping);
    app.observe(on_remove_dropping);
    app.observe(on_add_throwing);
    app.observe(on_remove_throwing);
    app.observe(on_add_holding);
    app.observe(on_remove_holding);
    app.observe(on_add_pulling);
    app.observe(on_remove_pulling);
}

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

fn on_add_pulling(_trigger: Trigger<OnAdd, Pulling>) {
    info!("Added: Pulling");
}

fn on_remove_pulling(_trigger: Trigger<OnRemove, Pulling>) {
    info!("Removed: Pulling");
}

fn on_add_holding(_trigger: Trigger<OnAdd, Holding>) {
    info!("Added: Holding");
}

fn on_remove_holding(_trigger: Trigger<OnRemove, Holding>) {
    info!("Removed: Holding");
}

fn on_add_throwing(_trigger: Trigger<OnAdd, Throwing>) {
    info!("Added: Throwing");
}

fn on_remove_throwing(_trigger: Trigger<OnRemove, Throwing>) {
    info!("Removed: Throwing");
}

fn on_add_dropping(_trigger: Trigger<OnAdd, Dropping>) {
    info!("Added: Dropping");
}

fn on_remove_dropping(_trigger: Trigger<OnRemove, Dropping>) {
    info!("Removed: Dropping");
}
