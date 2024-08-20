use crate::{prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(throw.in_set(HandleVerbSystem::Throw));
}

/// DetachObject
fn throw(mut commands: Commands, mut q_actor: Query<(Entity, &mut Cooldown, &Throwing)>) {
    for (actor, _cooldown, throw) in q_actor.iter_mut() {
        let _prop = throw.0;
        // Todo: cooldown.throw();
        info!("Throw!");
        commands.entity(actor).remove::<Throwing>();
        // TODO: Yeet object. This is also handled in DetachObject

        // TODO: let the user know this prop was dropped through an event or
        // observer. Do events sent in a fixed timestep get propagated
        // to `PostUpdate` even when two fixed update loops passed?
    }
}
