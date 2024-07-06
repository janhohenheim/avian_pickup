use std::iter;

use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::{prelude::*, utils::HashSet};

use crate::prelude::AvianPickupActor;

pub(super) fn prepare_spatial_query_filter(
    In(entity): In<Entity>,
    mut q_actor: Query<&mut AvianPickupActor>,
    q_parent: Query<&Parent>,
    q_collider: Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    q_rigid_body: Query<Has<RigidBody>>,
) {
    let Ok(mut actor) = q_actor.get_mut(entity) else {
        // Not doing unwrap here because we also run this on
        // entities that have their `ColliderConstructorHierarchy` removed,
        // so they might not have an `AvianPickupActor` component.
        return;
    };

    let excluded_entities = &mut actor
        .spatial_query_filter
        .excluded_entities;

    let rigid_body = iter::once(entity)
        .chain(q_parent.iter_ancestors(entity))
        .find(|&entity| q_rigid_body.contains(entity));
    if let Some(rigid_body) = rigid_body {
        collect_sub_colliders(rigid_body, &q_collider, excluded_entities);
    }
}

fn collect_sub_colliders(
    entity: Entity,
    q_collider: &Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    excluded_entities: &mut HashSet<Entity>,
) {
    // Unwrap cannot fail: the query only checks optional components
    let (is_collider, is_ancestor, children) = q_collider.get(entity).unwrap();
    if is_collider {
        excluded_entities.insert(entity);
    }
    if is_ancestor {
        if let Some(children) = children {
            for &child in children.iter() {
                collect_sub_colliders(child, q_collider, excluded_entities);
            }
        }
    }
}
