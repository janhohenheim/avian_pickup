use crate::prelude::*;

mod candidate;
mod find_in_cone;

use self::{candidate::*, find_in_cone::*};

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
    q_rigid_body: Query<(&RigidBody, &GlobalTransform)>,
) {
    let actor_entity = trigger.entity();
    let (origin, config) = q_actor.get(actor_entity).unwrap();

    let origin = origin.compute_transform();
    let candidate = get_object_candidate(&spatial_query, origin, config);

    let reaction = if let Some(candidate) = candidate {
        if candidate.toi_fraction < 0.25 {
            ObjectReaction::Hold
        } else {
            ObjectReaction::Pull
        }
    } else {
        ObjectReaction::None
    };

    if reaction == ObjectReaction::None || true {
        let object = find_object_in_cone(
            In(actor_entity),
            spatial_query,
            q_actor,
            q_collider,
            q_rigid_body,
        );
        info!("Found object: {object:?}");
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ObjectReaction {
    None,
    Pull,
    Hold,
}
