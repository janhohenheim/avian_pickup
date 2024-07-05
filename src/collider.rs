use std::iter;

use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::prelude::*;

use crate::prelude::AvianPickupActor;

pub(super) fn prepare_spatial_query_filter(
    In(entity): In<Entity>,
    q_actor: Query<&AvianPickupActor>,
    q_parent: Query<&Parent>,
    q_collider: Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    q_rigid_body: Query<Has<RigidBody>>,
) -> (Entity, SpatialQueryFilter) {
    let mut colliders = Vec::new();
    let rigid_body = iter::once(entity)
        .chain(q_parent.iter_ancestors(entity))
        .find(|&entity| q_rigid_body.contains(entity));
    if let Some(rigid_body) = rigid_body {
        collect_sub_colliders(rigid_body, &q_collider, &mut colliders);
    }
    let spatial_query_filter = q_actor
        .get(entity)
        .unwrap()
        .spatial_query_filter
        .clone()
        .with_excluded_entities(colliders);
    (entity, spatial_query_filter)
}

fn collect_sub_colliders(
    entity: Entity,
    q_collider: &Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    colliders: &mut Vec<Entity>,
) {
    // Unwrap cannot fail: the query only checks optional components
    let (is_collider, is_ancestor, children) = q_collider.get(entity).unwrap();
    if is_collider {
        colliders.push(entity);
    }
    if is_ancestor {
        if let Some(children) = children {
            for &child in children.iter() {
                collect_sub_colliders(child, q_collider, colliders);
            }
        }
    }
}
