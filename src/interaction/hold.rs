use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(hold.in_set(AvianPickupSystem::HoldObject));
}

fn hold(q_actor: Query<(&AvianPickupActorState, &GlobalTransform)>) {
    for (&state, transform) in q_actor.iter() {
        let AvianPickupActorState::Holding(_entity) = state else {
            continue;
        };
        let _transform = transform.compute_transform();
    }
}
