use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::prelude::*;
use dynamics::rigid_body;

pub(crate) const METERS_PER_INCH: f32 = 0.0254;

pub(crate) fn rigid_body_compound_collider(
    rigid_body: Entity,
    origin: Position,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<(&Position, &Rotation, &Collider), Without<Sensor>>,
) -> Collider {
    let mut colliders = Vec::new();
    rigid_body_compound_collider_recursive(
        rigid_body,
        q_collider_ancestor,
        q_collider,
        &mut colliders,
        origin,
    );
    Collider::compound(colliders)
}

fn rigid_body_compound_collider_recursive(
    candidate: Entity,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<(&Position, &Rotation, &Collider), Without<Sensor>>,
    colliders: &mut Vec<(Position, Rotation, Collider)>,
    origin: Position,
) {
    if let Ok((&pos, &rot, col)) = q_collider.get(candidate) {
        let pos = pos.0 - origin.0;
        colliders.push((pos.into(), rot, col.clone()));
    }
    if let Ok(children) = q_collider_ancestor.get(candidate) {
        for child in children.iter() {
            rigid_body_compound_collider_recursive(
                *child,
                q_collider_ancestor,
                q_collider,
                colliders,
                origin,
            );
        }
    }
}
