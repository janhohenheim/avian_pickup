use crate::{prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(throw.in_set(HandleVerbSystem::Throw));
}

fn throw(mut q_actor: Query<(&mut Cooldown, &Throwing)>) {
    for (_cooldown, throw) in q_actor.iter_mut() {
        let _prop = throw.0;
        // Todo: cooldown.throw();
        info!("Throw!");
    }
}
