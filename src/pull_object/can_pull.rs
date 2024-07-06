use crate::prelude::*;

/// Inspired by [`CWeaponPhysCannon::CanPickupObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L3421)
pub(super) fn can_pull(rigid_body: RigidBody, mass: Mass, config: &AvianPickupActor) -> bool {
    rigid_body == RigidBody::Dynamic && mass.0 < config.max_mass
}
