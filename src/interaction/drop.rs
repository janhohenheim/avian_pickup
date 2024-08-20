use crate::{prelude::*, verb::Dropping};

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(drop.in_set(HandleVerbSystem::Drop));
}

fn drop(mut q_state: Query<(&mut AvianPickupActorState, &mut Cooldown, &Dropping)>) {
    for (mut state, mut cooldown, drop) in q_state.iter_mut() {
        let _prop = drop.0;
        *state = AvianPickupActorState::Idle;
        info!("Drop!");
        cooldown.drop();
    }
}
