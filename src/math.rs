use avian3d::prelude::*;
use bevy::prelude::*;

pub(crate) const METERS_PER_INCH: f32 = 0.0254;

pub(crate) fn rigid_body_compound_collider(
    rigid_body_transform: Transform,
    colliders: Option<&RigidBodyColliders>,
    q_collider: &Query<(&GlobalTransform, &Collider, Option<&CollisionLayers>)>,
    filter: &SpatialQueryFilter,
) -> Option<Collider> {
    let collider_entities = colliders?;
    let colliders = collider_entities
        .iter()
        .filter_map(|e| {
            let (transform, collider, layers) = q_collider.get(e).ok()?;
            let transform = transform.compute_transform();
            let layers = layers.copied().unwrap_or_default();
            filter.test(e, layers).then(|| {
                (
                    transform.translation - rigid_body_transform.translation,
                    transform.rotation,
                    collider.clone(),
                )
            })
        })
        .collect::<Vec<_>>();

    (!colliders.is_empty()).then(|| Collider::compound(colliders))
}
