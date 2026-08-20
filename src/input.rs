//! Module for the types that represent input events for Avian Pickup.

use bevy_ecs::relationship::Relationship;

use crate::prelude::*;

pub(super) mod prelude {
    pub use super::{AvianPickupAction, AvianPickupInput};
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<AvianPickupInput>()
        .add_systems(PostUpdate, set_action_according_to_input);
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

fn set_action_according_to_input(
    mut r_input: MessageReader<AvianPickupInput>,
    mut commands: Commands,
    finder: PropFinder,
    mut q_actor: Query<(
        Option<&Holding>,
        &mut Cooldown,
        &AvianPickupActor,
        &GlobalTransform,
        Has<ShadowParams>,
        Has<HoldError>,
    )>,
    q_prop: Query<(Has<HeldBy>, &ComputedMass)>,
    mut pull: MessageWriter<PullRequest>,
    mut push: MessageWriter<PushRequest>,
) {
    'outer: for &event in r_input.read() {
        let action = event.action;
        let actor = event.actor;
        let Ok((holding, mut cooldown, config, actor_transform, has_shadow, has_error)) =
            q_actor.get_mut(actor)
        else {
            error!(
                "`AvianPickupEvent` was triggered on an entity without `AvianPickupActor`. Ignoring."
            );
            continue;
        };

        // Doing these checks now so that we can report issues early.
        let checks = [(has_shadow, "ShadowParams"), (has_error, "HoldError")];
        for (has_component, component_name) in checks.iter() {
            if !has_component {
                error!(
                    "`AvianPickupEvent` was triggered on an entity without `{component_name}`. Ignoring."
                );
                continue 'outer;
            }
        }

        let mut actor_commands = commands.entity(actor);
        match action {
            AvianPickupAction::Throw
                if let Some(holding) = holding
                    && cooldown.is_finished(AvianPickupAction::Throw) =>
            {
                actor_commands.remove::<Holding>();
                cooldown.throw();

                push.write(PushRequest {
                    actor,
                    prop: holding.get(),
                });
            }
            AvianPickupAction::Drop
                if holding.is_some() && cooldown.is_finished(AvianPickupAction::Drop) =>
            {
                actor_commands.remove::<Holding>();
                cooldown.drop();
            }
            AvianPickupAction::Pull
                if holding.is_none() && cooldown.is_finished(AvianPickupAction::Pull) =>
            {
                cooldown.pull();
                let Some(find_prop) = finder.find_prop(*actor_transform, config) else {
                    continue;
                };

                let prop = find_prop.entity;
                let Ok((is_already_being_held, &mass)) = q_prop.get(prop) else {
                    continue;
                };

                if is_already_being_held || mass.value() >= config.pull.max_prop_mass {
                    continue;
                }

                let can_hold = find_prop.toi <= config.hold.distance_to_allow_holding;
                if can_hold {
                    cooldown.hold();
                    actor_commands.insert(Holding(find_prop.entity));
                    continue;
                }

                pull.write(PullRequest {
                    actor,
                    prop: find_prop.entity,
                });
            }
            _ => {}
        };
    }
}
