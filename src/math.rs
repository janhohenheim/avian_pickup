use avian3d::{prelude::*, sync::ancestor_marker::AncestorMarker};
use bevy::prelude::*;

pub(crate) const METERS_PER_INCH: f32 = 0.0254;

pub(crate) fn rigid_body_compound_collider(
    rigid_body: Entity,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<(&Transform, &Collider), Without<Sensor>>,
) -> Option<Collider> {
    let mut colliders = Vec::new();
    if let Ok((&_transform, col)) = q_collider.get(rigid_body) {
        colliders.push((Vec3::ZERO, Quat::IDENTITY, col.clone()));
    }
    if let Ok(children) = q_collider_ancestor.get(rigid_body) {
        for child in children.iter() {
            rigid_body_compound_collider_recursive(
                *child,
                q_collider_ancestor,
                q_collider,
                &mut colliders,
            );
        }
    }
    if colliders.is_empty() {
        None
    } else {
        Some(Collider::compound(colliders))
    }
}

fn rigid_body_compound_collider_recursive(
    candidate: Entity,
    q_collider_ancestor: &Query<&Children, With<AncestorMarker<ColliderMarker>>>,
    q_collider: &Query<(&Transform, &Collider), Without<Sensor>>,
    colliders: &mut Vec<(Vec3, Quat, Collider)>,
) {
    if let Ok((&transform, col)) = q_collider.get(candidate) {
        colliders.push((transform.translation, transform.rotation, col.clone()));
    }
    if let Ok(children) = q_collider_ancestor.get(candidate) {
        for child in children.iter() {
            rigid_body_compound_collider_recursive(
                *child,
                q_collider_ancestor,
                q_collider,
                colliders,
            );
        }
    }
}

pub(crate) trait GetBestGlobalTransform {
    fn get_best_global_transform(&self, entity: Entity) -> Transform;
}

impl GetBestGlobalTransform
    for Query<'_, '_, (&GlobalTransform, Option<&Position>, Option<&Rotation>)>
{
    fn get_best_global_transform(&self, entity: Entity) -> Transform {
        let (global_transform, position, rotation) = self.get(entity).unwrap();
        if let Some(position) = position {
            if let Some(rotation) = rotation {
                return Transform::from_translation(position.0).with_rotation(rotation.0);
            }
        }
        global_transform.compute_transform()
    }
}
