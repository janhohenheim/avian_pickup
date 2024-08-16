use bevy::prelude::*;

use super::OnHold;
use crate::prelude::*;

/// CGrabController::AttachEntity
pub(super) fn on_hold(
    trigger: Trigger<OnHold>,
    mut q_actor: Query<(&mut AvianPickupActorState,)>,
    q_prop: Query<(Option<&PreferredPickupRotation>, &Rotation)>,
) {
    let actor_entity = trigger.entity();
    let prop_entity = trigger.event().0;
    let (mut state,) = q_actor.get_mut(actor_entity).unwrap();

    *state = AvianPickupActorState::Holding(prop_entity);
    // Safety: All props are rigid bodies, so they are guaranteed to have a
    // `Rotation`.
    let (preferred_rotation, rotation) = q_prop.get(prop_entity).unwrap();
    let target_rotation = preferred_rotation
        .map(|preferred| preferred.0)
        .unwrap_or(rotation.0);
}
