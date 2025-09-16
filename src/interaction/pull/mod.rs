use crate::{
    prelude::*,
    verb::{Pulling, SetVerb, Verb},
};

mod can_pull;
mod find_in_cone;
mod find_in_trace;

use self::{can_pull::*, find_in_cone::*, find_in_trace::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PhysicsSchedule, find_object.in_set(HandleVerbSystem::Pull))
        .add_systems(
            PhysicsSchedule,
            flush_pulling_state.in_set(AvianPickupSystem::ResetIdle),
        );
}

/// Inspired by [`CWeaponPhysCannon::FindObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/server/hl2/weapon_physcannon.cpp#L2497)
fn find_object(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut q_actor: Query<
        (
            Entity,
            &GlobalTransform,
            &AvianPickupActor,
            &mut AvianPickupActorState,
            &mut Cooldown,
        ),
        With<Pulling>,
    >,
    q_collider_parent: Query<&ColliderOf>,
    mut q_rigid_body: Query<(
        &RigidBody,
        &ComputedMass,
        Forces,
        &GlobalTransform,
        Has<HeldProp>,
    )>,
    q_position: Query<&GlobalTransform>,
) {
    for (actor, actor_transform, config, mut state, mut cooldown) in q_actor.iter_mut() {
        let actor_transform = actor_transform.compute_transform();
        let prop = find_prop_in_trace(
            &spatial_query,
            actor_transform,
            config,
            &q_rigid_body.transmute_lens().query(),
            &q_collider_parent,
        )
        .or_else(|| {
            find_prop_in_cone(
                &spatial_query,
                actor_transform,
                config,
                &q_position,
                &q_rigid_body.transmute_lens().query(),
                &q_collider_parent,
            )
        });
        let Some(prop) = prop else {
            continue;
        };

        let Ok((_, &mass, mut forces, prop_position, is_already_being_held)) =
            q_rigid_body.get_mut(prop.entity)
        else {
            // These components might not be present on non-dynamic rigid bodies
            continue;
        };

        if is_already_being_held || !can_pull(mass, config) {
            continue;
        }

        let can_hold = prop.toi <= config.hold.distance_to_allow_holding;
        if can_hold {
            cooldown.hold();
            commands
                .entity(actor)
                .queue(SetVerb::new(Verb::Hold(prop.entity)));
        } else {
            let direction =
                (actor_transform.translation - prop_position.translation()).normalize_or_zero();
            let mass_adjustment = adjust_impulse_for_mass(mass);
            let pull_impulse = direction * config.pull.impulse * mass_adjustment;
            cooldown.pull();
            forces.apply_linear_impulse(pull_impulse);
            if !matches!(state.as_ref(), AvianPickupActorState::Pulling(..)) {
                *state = AvianPickupActorState::Pulling(prop.entity);
            }
            commands.entity(actor).queue(SetVerb::new(None));
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]

struct Prop {
    pub entity: Entity,
    pub toi: f32,
}

fn flush_pulling_state(mut q_state: Query<(Mut<AvianPickupActorState>, Has<Pulling>, &Cooldown)>) {
    for (mut state, has_pulling, cooldown) in q_state.iter_mut() {
        // Okay, so the basic idea is this:
        // Pulling happens in discrete impulses every n milliseconds.
        // New pulls happen regularly, but we should also reset to idle at some point.
        // Technically, we could reset to idle every frame and let it be overwritten by
        // a new pull, but that would mean that in the time between discrete
        // impulses, the state would be idle. That is technically true, but
        // probably not what a user observing the state component would expect.
        // So, instead we set it to idle only if the cooldown is finished.
        //
        // The reason we check for `!has_pulling` is that a missing `Pulling` means
        // that no input was given to start / continue pulling during `Update`.
        if matches!(state.as_ref(), AvianPickupActorState::Pulling(..))
            && !has_pulling
            && cooldown.is_finished(AvianPickupAction::Pull)
        {
            *state = AvianPickupActorState::Idle;
        }
    }
}
