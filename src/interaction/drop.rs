use avian3d::math::{Scalar, TAU};

use crate::{prelude::*, verb::Dropping};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, drop.in_set(HandleVerbSystem::Drop));
}

/// DetachObject
fn drop(
    mut commands: Commands,
    mut q_actor: Query<(Entity, &mut AvianPickupActorState, &mut Cooldown, &Dropping)>,
    mut q_prop: Query<(&mut LinearVelocity, &mut AngularVelocity)>,
    mut w_drop_event: MessageWriter<PropDropped>,
) {
    for (actor, mut state, mut cooldown, drop) in q_actor.iter_mut() {
        let prop = drop.prop;
        *state = AvianPickupActorState::Idle;
        cooldown.drop();
        commands.entity(actor).remove::<Dropping>();
        w_drop_event.write(PropDropped {
            actor,
            prop,
            forced: drop.forced,
        });
        // Safety: the prop is a dynamic rigid body and thus is guaranteed to have a
        // linvel and angvel.
        let Ok((mut velocity, mut angvel)) = q_prop.get_mut(prop) else {
            error!("Prop entity was deleted or in an invalid state. Ignoring.");
            continue;
        };
        // HL2 uses 190 inches per second, which is 4.826 meters per second.
        // let's round that to 5 m/s.
        const HL2_NORM_SPEED: Scalar = 5.0;
        const MAX_DROP_LINEAR_SPEED: Scalar = HL2_NORM_SPEED * 1.5;
        const MAX_DROP_ANGULAR_SPEED: Scalar = TAU * 2.0;
        velocity.0 = velocity.clamp_length_max(MAX_DROP_LINEAR_SPEED);
        angvel.0 = angvel.clamp_length_max(MAX_DROP_ANGULAR_SPEED);
    }
}
