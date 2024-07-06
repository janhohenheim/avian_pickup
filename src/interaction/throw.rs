use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(throw);
}

#[derive(Debug, Event)]
pub(crate) struct ThrowObject;

fn throw(trigger: Trigger<ThrowObject>, mut q_cooldown: Query<&mut Cooldown>) {
    let actor_entity = trigger.entity();
    let cooldown = q_cooldown.get_mut(actor_entity).unwrap();
    if !cooldown.left.finished() {
        return;
    }
    // Todo: cooldown.throw();
    info!("Throw!");
}
