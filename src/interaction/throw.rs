use crate::{prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, throw.in_set(HandleVerbSystem::Throw));
}

/// DetachObject
fn throw(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &mut Cooldown, &Throwing)>,
    mut w_throw_event: EventWriter<PropThrown>,
) {
    for (actor, mut cooldown, throw) in q_actor.iter_mut() {
        let prop = throw.0;
        info!("Throw!");
        commands.entity(actor).remove::<Throwing>();
        if let Some(prop) = prop {
            // TODO: Yeet object. This is also handled in DetachObject through
            // PrimaryAttack
            commands.entity(prop).remove::<HeldProp>();
            w_throw_event.send(PropThrown {
                actor,
                prop,
                was_held: true,
            });
        } else {
            // YTODO: eet next object in front of us

            w_throw_event.send(PropThrown {
                actor,
                prop: Entity::PLACEHOLDER,
                was_held: false,
            });
        }

        // TODO: only CD when we actually threw something
        cooldown.throw();
    }
}
