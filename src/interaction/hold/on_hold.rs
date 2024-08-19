use bevy::prelude::*;

use super::GrabParams;
use crate::{prelude::*, prop::PickupMass, verb::Holding};

/// CGrabController::AttachEntity
pub(super) fn on_hold(
    trigger: Trigger<OnAdd, Holding>,
    mut commands: Commands,
    mut q_actor: Query<(&mut AvianPickupActorState, &mut GrabParams, &Holding)>,
    mut q_prop: Query<(Option<&PickupMass>, &mut Mass)>,
) {
    let actor = trigger.entity();
    let (mut state, mut grab, holding) = q_actor.get_mut(actor).unwrap();
    let prop = holding.0;
    *state = AvianPickupActorState::Holding(prop);
    // Safety: All props are rigid bodies, so they are guaranteed to have a
    // `Rotation` and `Rotation`.
    let (pickup_mass, mut mass) = q_prop.get_mut(prop).unwrap();
    let new_mass = pickup_mass.copied().unwrap_or_default().0;
    commands.entity(prop).insert(NonPickupMass(mass.0));
    mass.0 = new_mass;
    // The original code also does some damping stuff, but then deactivates
    // drag? Seems like a no-op to me

    // 1 second until error starts accumulating
    grab.error_time = -1.0;

    // The original code now does some stuff with `AlignAngles`, but it only
    // does so when `m_angleAlignment != 0`, which does not seem to be the
    // case for HL2 deathmatch, judging by the code? Anyhoot, per
    // discussions on Discord, that code seems to align the prop to
    // the coordinate axes if it is closer than 30 degrees to them.
    // Does not seem to be that useful.
}
