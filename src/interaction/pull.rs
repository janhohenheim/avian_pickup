use crate::{AvianPickupSystem::HandlePull, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, procees_pull_requests.in_set(HandlePull))
        .add_message::<PullRequest>()
        .add_message::<PropPulled>();
}

/// Message to start a pull operation.
#[derive(Message, Debug, Clone, Copy)]
pub struct PullRequest {
    /// [`AvianPickupActor`] that will pull the prop.
    pub actor: Entity,
    /// Dynamic [`RigidBody`] to pull.
    pub prop: Entity,
}

impl PullRequest {
    fn done(self, impulse: Vec3) -> PropPulled {
        PropPulled {
            actor: self.actor,
            prop: self.prop,
            impulse,
        }
    }
}

/// Message sent when prop has being pulled with success.
#[derive(Message, Debug, Clone, Copy)]
pub struct PropPulled {
    /// [`AvianPickupActor`] that pulled the prop.
    pub actor: Entity,
    /// Dynamic [`RigidBody`] pulled by the actor.
    pub prop: Entity,
    /// The impulse applied to the prop.
    pub impulse: Vec3,
}

fn procees_pull_requests(
    mut requests: MessageReader<PullRequest>,
    mut pulled: MessageWriter<PropPulled>,
    q_actor: Query<(&GlobalTransform, &AvianPickupActor)>,
    mut q_rigid_body: Query<(&ComputedMass, Forces, &GlobalTransform), With<RigidBody>>,
) {
    for request in requests.read() {
        let actor = request.actor;
        let Ok((actor_transform, config)) = q_actor.get(actor) else {
            continue;
        };

        let prop = request.prop;
        let Ok((&mass, mut forces, prop_transform)) = q_rigid_body.get_mut(prop) else {
            continue;
        };

        let actor_pos = actor_transform.translation();
        let prop_pos = prop_transform.translation();

        let direction = (actor_pos - prop_pos).normalize_or_zero();
        let mass_adjustment = adjust_impulse_for_mass(mass);
        let pull_impulse = direction * config.pull.impulse * mass_adjustment;
        forces.apply_linear_impulse(pull_impulse);
        pulled.write(request.done(pull_impulse));
    }
}

/// Taken from [this snippet](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2607-L2610)
fn adjust_impulse_for_mass(mass: ComputedMass) -> f32 {
    if mass.value() < 50.0 {
        (mass.value() + 0.5) * (1.0 / 50.0)
    } else {
        1.0
    }
}
