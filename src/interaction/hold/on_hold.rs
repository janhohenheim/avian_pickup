use bevy::prelude::*;

use super::OnHold;
use crate::prelude::*;

pub(super) fn on_hold(trigger: Trigger<OnHold>, mut q_actor: Query<(&mut AvianPickupActorState,)>) {
    let entity = trigger.entity();
    let (mut state,) = q_actor.get_mut(entity).unwrap();

    *state = AvianPickupActorState::Holding(entity);
}
