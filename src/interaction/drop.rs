use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(drop);
}

#[derive(Debug, Event)]
pub(crate) struct DropObject;

fn drop(
    trigger: Trigger<DropObject>,
    mut q_state: Query<(&mut AvianPickupActorState, &mut Cooldown)>,
) {
    let actor_entity = trigger.entity();
    let (mut state, mut cooldown) = q_state.get_mut(actor_entity).unwrap();
    if !cooldown.right.finished() {
        return;
    }
    *state = AvianPickupActorState::Idle;
    cooldown.drop();
}
