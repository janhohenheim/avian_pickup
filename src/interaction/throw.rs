use crate::{prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(throw.in_set(HandleVerbSystem::Throw));
}

fn throw(mut q_actor: Query<&mut Cooldown, With<Throwing>>) {
    for _cooldown in q_actor.iter_mut() {
        // Todo: cooldown.throw();
        info!("Throw!");
    }
}
