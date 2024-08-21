use crate::{prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(throw.in_set(HandleVerbSystem::Throw));
}

/// DetachObject
fn throw(mut commands: Commands, mut q_actor: Query<(Entity, &mut Cooldown, &Throwing)>) {
    for (actor, mut cooldown, throw) in q_actor.iter_mut() {
        let _prop = throw.0;
        info!("Throw!");
        commands.entity(actor).remove::<Throwing>();
        // TODO: Yeet object. This is also handled in DetachObject through
        // PrimaryAttack

        // TODO: only CD when we actually threw something
        cooldown.throw();

        // TODO: let the user know this prop was dropped through an event or
        // observer. Do events sent in a fixed timestep get propagated
        // to `PostUpdate` even when two fixed update loops passed?
    }
}
