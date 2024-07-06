use crate::prelude::*;

mod can_pull;
mod candidate;
mod find_in_cone;

use self::{can_pull::*, candidate::*, find_in_cone::*};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PullObject>().observe(find_object);
}

#[derive(Debug, Event)]
pub(crate) struct PullObject;

/// Inspired by [`CWeaponPhysCannon::FindObject`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/game/server/hl2/weapon_physcannon.cpp#L2497)
fn find_object(
    trigger: Trigger<PullObject>,
    spatial_query: SpatialQuery,
    q_actor: Query<(&GlobalTransform, &AvianPickupActor)>,
    q_collider: Query<&ColliderParent>,
    q_rigid_body: Query<(&RigidBody, Option<&Mass>)>,
    q_transform: Query<&GlobalTransform>,
) {
    let actor_entity = trigger.entity();
    let (origin, config) = q_actor.get(actor_entity).unwrap();

    let origin = origin.compute_transform();
    let candidate = get_object_candidate(&spatial_query, origin, config)
        .or_else(|| find_object_in_cone(&spatial_query, origin, &config, &q_transform));

    let Some(candidate) = candidate else {
        return;
    };
    if !can_pull(candidate.entity, config, &q_collider, &q_rigid_body) {
        return;
    }
    let can_hold = candidate.toi <= config.trace_length;
    info!("{candidate:?} can be held: {can_hold}");
}

#[derive(Debug, Clone, Copy, PartialEq)]

struct Candidate {
    pub entity: Entity,
    pub toi: f32,
}
