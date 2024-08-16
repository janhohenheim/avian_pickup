use bevy::prelude::*;

use super::{GrabParams, OnHold};
use crate::{prelude::*, prop::PickupMass};

/// CGrabController::AttachEntity
pub(super) fn on_hold(
    trigger: Trigger<OnHold>,
    mut commands: Commands,
    mut q_actor: Query<(&mut AvianPickupActorState, &mut GrabParams)>,
    mut q_prop: Query<(
        Option<&PreferredPickupRotation>,
        Option<&PreferredPickupDistance>,
        Option<&PickupMass>,
        &mut Mass,
        &Rotation,
    )>,
) {
    let actor_entity = trigger.entity();
    let prop_entity = trigger.event().0;
    let (mut state, mut grab) = q_actor.get_mut(actor_entity).unwrap();

    *state = AvianPickupActorState::Holding(prop_entity);
    // Safety: All props are rigid bodies, so they are guaranteed to have a
    // `Rotation` and `Rotation`.
    let (preferred_rotation, preferred_distance, pickup_mass, mut mass, rotation) =
        q_prop.get_mut(prop_entity).unwrap();
    let target_rotation = preferred_rotation
        .map(|preferred| preferred.0)
        .unwrap_or(rotation.0);
    let target_distance = preferred_distance.copied().unwrap_or_default().0;
    let new_mass = pickup_mass.copied().unwrap_or_default().0;
    commands.entity(prop_entity).insert(NonPickupMass(mass.0));
    mass.0 = new_mass;
    // The original code also does some damping stuff, but then deactivates
    // drag? Seems like a no-op to me

    grab.error_time = 1.0;

    // The original code now does some stuff with `AlignAngles`, but it only
    // does so when `m_angleAlignment != 0`, which does not seem to be the
    // case for HL2 deathmatch, judging by the code? Anyhoot, per discussions
    // on Discord, that code seems to align the prop to the coordinate axes
    // if it is closer than 30 degrees to them. Does not seem to be that useful.
}
