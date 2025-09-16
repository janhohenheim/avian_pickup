use super::prelude::HoldError;
use crate::{prelude::*, prop::PrePickupRotation, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_holding);
}

/// CGrabController::AttachEntity
pub fn on_add_holding(
    trigger: On<Add, Holding>,
    mut commands: Commands,
    mut q_actor: Query<(
        &AvianPickupActor,
        &mut AvianPickupActorState,
        &mut HoldError,
        &Holding,
    )>,
    q_actor_transform: Query<&GlobalTransform>,
    mut q_prop: Query<(
        &GlobalTransform,
        Option<&Mass>,
        Option<&PickupMassOverride>,
        Option<&mut PrePickupRotation>,
    )>,
) {
    let actor = trigger.entity;
    let Ok((config, mut state, mut hold_error, holding)) = q_actor.get_mut(actor) else {
        error!("Actor entity was deleted or in an invalid state. Ignoring.");
        return;
    };
    let Ok(actor_transform) = q_actor_transform
        .get(actor)
        .map(|transform| transform.compute_transform())
    else {
        error!("Actor entity was deleted or in an invalid state. Ignoring.");
        return;
    };
    let prop = holding.0;
    *state = AvianPickupActorState::Holding(prop);
    commands.entity(prop).try_insert(HeldProp);
    let Ok((prop_transform, mass, pickup_mass, pre_pickup_rotation)) = q_prop.get_mut(prop) else {
        error!("Prop entity was deleted or in an invalid state. Ignoring.");
        return;
    };

    let actor_space_rotation =
        prop_rotation_to_actor_space(prop_transform.rotation(), actor_transform);
    if let Some(mut pre_pickup_rotation) = pre_pickup_rotation {
        pre_pickup_rotation.0 = actor_space_rotation;
    } else {
        commands
            .entity(prop)
            .try_insert(PrePickupRotation(actor_space_rotation));
    }

    // Cache old mass
    if let Some(mass) = mass {
        commands.entity(prop).try_insert(NonPickupMass(*mass));
    }

    let new_mass = pickup_mass
        .map(|m| m.0)
        .unwrap_or(config.hold.temporary_prop_mass);

    commands.entity(prop).try_insert(Mass(new_mass));

    // The original code also does some damping stuff, but then deactivates
    // drag? Seems like a no-op to me

    hold_error.reset();

    // The original code now does some stuff with `AlignAngles`, but it only
    // does so when `m_angleAlignment != 0`, which does not seem to be the
    // case for HL2 deathmatch, judging by the code? Anyhoot, per
    // discussions on Discord, that code seems to align the prop to
    // the coordinate axes if it is closer than 30 degrees to them.
    // Does not seem to be that useful.
}

/// TransformAnglesToPlayerSpace
fn prop_rotation_to_actor_space(rot: Quat, actor: Transform) -> Quat {
    let world_to_actor = actor.compute_affine().inverse();
    let rot_to_world = Transform::from_rotation(rot).compute_affine();
    let local_affine = world_to_actor * rot_to_world;
    Quat::from_affine3(&local_affine)
}
