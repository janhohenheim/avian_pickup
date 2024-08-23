use crate::{prelude::*, verb::Dropping};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, drop.in_set(HandleVerbSystem::Drop));
}

/// DetachObject
fn drop(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &mut AvianPickupActorState, &mut Cooldown, &Dropping)>,
    mut q_prop: Query<(&mut LinearVelocity, &mut AngularVelocity)>,
    mut w_drop_event: EventWriter<PropDropped>,
) {
    for (actor, mut state, mut cooldown, drop) in q_actor.iter_mut() {
        let prop = drop.prop;
        *state = AvianPickupActorState::Idle;
        cooldown.drop();
        commands.entity(actor).remove::<Dropping>();
        w_drop_event.send(PropDropped {
            actor,
            prop,
            forced: drop.forced,
        });
        // Safety: the prop is a dynamic rigid body and thus is guaranteed to have a
        // linvel and angvel.
        let (mut velocity, mut angvel) = q_prop.get_mut(prop).unwrap();
        velocity.0 = Vec3::ZERO;
        angvel.0 = Vec3::ZERO;
    }
}
