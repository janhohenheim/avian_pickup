use std::iter;

use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::{prelude::*, utils::HashSet};

use crate::prelude::AvianPickupActor;

pub(super) fn plugin(app: &mut App) {}

#[derive(Debug, Default, Component)]
pub(crate) struct AssociatedColliders(pub(crate) HashSet<Entity>);

fn update_associated_colliders(
    mut q_actor: Query<(Entity, &mut AssociatedColliders), With<AvianPickupActor>>,
    q_parent: Query<&Parent>,
    q_collider: Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    q_rigid_body: Query<Has<RigidBody>>,
) {
    for (entity, mut associated_colliders) in q_actor.iter_mut() {
        associated_colliders.0.clear();
        let rigid_body = iter::once(entity)
            .chain(q_parent.iter_ancestors(entity))
            .find(|&entity| q_rigid_body.contains(entity));
        if let Some(rigid_body) = rigid_body {
            collect_sub_colliders(rigid_body, &q_collider, &mut associated_colliders.0);
        }
    }
}

fn collect_sub_colliders(
    entity: Entity,
    q_collider: &Query<(
        Has<Collider>,
        Has<AncestorMarker<Collider>>,
        Option<&Children>,
    )>,
    colliders: &mut HashSet<Entity>,
) {
    // Unwrap cannot fail: the query only checks optional components
    let (is_collider, is_ancestor, children) = q_collider.get(entity).unwrap();
    if is_collider {
        colliders.insert(entity);
    }
    if is_ancestor {
        if let Some(children) = children {
            for &child in children.iter() {
                collect_sub_colliders(child, q_collider, colliders);
            }
        }
    }
}
