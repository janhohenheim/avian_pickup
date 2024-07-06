use crate::prelude::*;

mod can_pull;
mod find_in_cone;
mod find_in_trace;

use self::{can_pull::*, find_in_cone::*, find_in_trace::*};

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
    mut q_rigid_body: Query<(&RigidBody, &Mass, &mut ExternalImpulse, &GlobalTransform)>,
    q_transform: Query<&GlobalTransform>,
) {
    let actor_entity = trigger.entity();
    let (origin, config) = q_actor.get(actor_entity).unwrap();

    let origin = origin.compute_transform();
    let candidate = find_object_in_trace(&spatial_query, origin, config)
        .or_else(|| find_object_in_cone(&spatial_query, origin, &config, &q_transform));

    let Some(candidate) = candidate else {
        return;
    };

    // unwrap cannot fail: all colliders have a `ColliderParent`
    let rigid_body_entity = q_collider.get(candidate.entity).unwrap().get();

    let Ok((&rigid_body, &mass, mut impulse, object_transform)) =
        q_rigid_body.get_mut(rigid_body_entity)
    else {
        // These components might not be present on non-dynamic rigid bodies
        return;
    };

    if !can_pull(rigid_body, mass, &config) {
        return;
    }

    let can_hold = candidate.toi <= config.trace_length;
    info!("{candidate:?} can be held: {can_hold}");
    if !can_hold {
        let object_transform = object_transform.compute_transform();
        let direction = origin.translation - object_transform.translation;
        let magic_factor_ask_valve = if mass.0 < 50.0 {
            (mass.0 + 0.5) * (1.0 / 50.0)
        } else {
            1.0
        };
        let pull_impulse = direction * config.pull_force * magic_factor_ask_valve;
        info!("Applying pull impulse: {pull_impulse}");
        impulse.apply_impulse(pull_impulse);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

struct Candidate {
    pub entity: Entity,
    pub toi: f32,
}
