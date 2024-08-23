use super::prelude::HoldError;
use crate::{math::GetBestGlobalTransform, prelude::*, prop::PrePickupRotation, verb::Holding};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_add_holding);
}

/// CGrabController::AttachEntity
pub fn on_add_holding(
    trigger: Trigger<OnAdd, Holding>,
    mut commands: Commands,
    mut q_actor: Query<(
        &AvianPickupActor,
        &mut AvianPickupActorState,
        &mut HoldError,
        &Holding,
    )>,
    q_actor_transform: Query<(&GlobalTransform, Option<&Position>, Option<&Rotation>)>,
    mut q_prop: Query<(
        &Rotation,
        &mut Mass,
        Option<&PickupMassOverride>,
        Option<&mut NonPickupMass>,
        Option<&mut PrePickupRotation>,
    )>,
) {
    let actor = trigger.entity();
    let (config, mut state, mut hold_error, holding) = q_actor.get_mut(actor).unwrap();
    let actor_transform = q_actor_transform.get_best_global_transform(actor);
    let prop = holding.0;
    *state = AvianPickupActorState::Holding(prop);
    commands.entity(prop).insert(HeldProp);
    // Safety: All props are rigid bodies, so they are guaranteed to have a
    // `Rotation` and `Mass`.
    let (rotation, mut mass, pickup_mass, non_pickup_mass, pre_pickup_rotation) =
        q_prop.get_mut(prop).unwrap();
    let new_mass = pickup_mass.map(|m| m.0).unwrap_or(config.pickup_mass);
    if let Some(mut non_pickup_mass) = non_pickup_mass {
        non_pickup_mass.0 = mass.0;
    } else {
        // This has some overhead, even if it only overwrites the existing component,
        // so let's try to avoid it if possible
        commands.entity(prop).insert(NonPickupMass(mass.0));
    }

    let actor_space_rotation = prop_rotation_to_actor_space(rotation.0, actor_transform);
    if let Some(mut pre_pickup_rotation) = pre_pickup_rotation {
        pre_pickup_rotation.0 = actor_space_rotation;
    } else {
        commands
            .entity(prop)
            .insert(PrePickupRotation(actor_space_rotation));
    }

    mass.0 = new_mass;
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
