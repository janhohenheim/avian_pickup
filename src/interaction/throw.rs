use crate::{math::GetBestGlobalTransform, prelude::*, verb::Throwing};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, throw.in_set(HandleVerbSystem::Throw));
}

/// DetachObject
fn throw(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &AvianPickupActor, &mut Cooldown, &Throwing)>,
    q_actor_transform: Query<(&GlobalTransform, Option<&Position>, Option<&Rotation>)>,
    mut q_prop: Query<(&mut LinearVelocity, &mut AngularVelocity, &Position)>,
    mut w_throw_event: EventWriter<PropThrown>,
) {
    for (actor, config, mut cooldown, throw) in q_actor.iter_mut() {
        let prop = throw.0;
        info!("Throw!");
        commands.entity(actor).remove::<Throwing>();
        if let Some(prop) = prop {
            // TODO: Yeet object. This is also handled in DetachObject through
            // PrimaryAttack
            let actor_transform = q_actor_transform.get_best_global_transform(actor);
            // Safety: All props are rigid bodies, which are guaranteed to have a
            // `Position`.
            let (mut velocity, mut angvel, prop_position) = q_prop.get_mut(prop).unwrap();
            let prop_dist_sq = actor_transform
                .translation
                .distance_squared(prop_position.0);
            if prop_dist_sq > config.trace_length * config.trace_length {
                // Note: I don't think this will ever happen, but the 2013 code
                // does this check, so let's keep it just in case.
                continue;
            }

            velocity.0 = Vec3::ZERO;
            angvel.0 = Vec3::ZERO;
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
