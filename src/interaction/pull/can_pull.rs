use crate::prelude::*;

/// Inspired by [`CWeaponPhysCannon::CanPickupObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L3421)
pub(super) fn can_pull(mass: ComputedMass, config: &AvianPickupActor) -> bool {
    mass.value() < config.pull.max_prop_mass
}
