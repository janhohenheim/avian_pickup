use crate::prelude::*;

/// Inspired by [`CWeaponPhysCannon::CanPickupObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L3421)
pub(super) fn can_pull(
    entity: Entity,
    config: &AvianPickupActor,
    q_collider: &Query<&ColliderParent>,
    q_rigid_body: &Query<(&RigidBody, Option<&Mass>)>,
) -> bool {
    // unwrap cannot fail: all colliders have a `ColliderParent`
    let rigid_body_entity = q_collider.get(entity).unwrap().get();
    // unwrap cannot fail: `ColliderParent` always points to a `RigidBody`
    let (&rigid_body, mass) = q_rigid_body.get(rigid_body_entity).unwrap();
    // Note: currently, all `RigidBody`s have a `Mass`, but in the future,
    // `RigidBody::Static` might not.
    rigid_body == RigidBody::Dynamic && mass.is_some_and(|m| m.0 < config.max_mass)
}
