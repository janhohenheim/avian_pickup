use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::prelude::*;

pub(crate) fn get_rigid_body_colliders(
    rigid_body: Entity,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<&Collider>,
) -> Option<Vec<Entity>> {
    let mut colliders = Vec::new();
    get_rigid_body_colliders_recursive(rigid_body, q_collider_ancestor, q_collider, &mut colliders);
    if colliders.is_empty() {
        None
    } else {
        Some(colliders)
    }
}

fn get_rigid_body_colliders_recursive(
    entity: Entity,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<&Collider>,
    colliders: &mut Vec<Entity>,
) {
    if q_collider.contains(entity) {
        colliders.push(entity);
    }
    if let Ok(children) = q_collider_ancestor.get(entity) {
        for child in children.iter() {
            get_rigid_body_colliders_recursive(child, q_collider_ancestor, q_collider, colliders);
        }
    }
}
