use bevy::prelude::*;

use crate::{prelude::AvianPickupActorState, pull_object::PullObject};

pub(super) mod prelude {
    pub use super::AvianPickupEvent;
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupEvent>()
        .add_event::<AvianPickupEvent>()
        .observe(usher_event);
}

/// Event for picking up and throwing objects.
/// Send this to tell Avian Pickup to do its thing.
#[derive(Event, Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum AvianPickupEvent {
    /// The left mouse button was just pressed this update.
    JustPressedL,
    /// The right mouse button was just pressed this update.
    JustPressedR,
    /// The right mouse button was pressed.
    PressedR,
}

fn usher_event(
    trigger: Trigger<AvianPickupEvent>,
    mut commands: Commands,
    q_actor: Query<&AvianPickupActorState>,
) {
    let event = trigger.event();
    let entity = trigger.entity();
    let Ok(&state) = q_actor.get(entity) else {
        error!(
            "`AvianPickupEvent` was triggered on an entity without `AvianPickupActorState`. Ignoring."
        );
        return;
    };

    match event {
        AvianPickupEvent::JustPressedL => info!("Throw"),
        AvianPickupEvent::JustPressedR if state == AvianPickupActorState::Holding => info!("Drop"),
        AvianPickupEvent::JustPressedR | AvianPickupEvent::PressedR => {
            if state != AvianPickupActorState::Holding {
                commands.trigger_targets(PullObject, entity)
            }
        }
    }
}
