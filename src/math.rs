use avian3d::prelude::*;
use bevy::prelude::*;

pub(crate) const METERS_PER_INCH: f32 = 0.0254;

pub(crate) fn rigid_body_compound_collider(
    rigid_body_position: Position,
    colliders: Option<&RigidBodyColliders>,
    q_collider: &Query<(&Transform, &Collider, Option<&CollisionLayers>)>,
    filter: &SpatialQueryFilter,
) -> Option<Collider> {
    let collider_entities = colliders?;
    let colliders = collider_entities
        .iter()
        .filter_map(|e| {
            let (transform, collider, layers) = q_collider.get(e).ok()?;
            let layers = layers.copied().unwrap_or_default();
            filter.test(e, layers).then(|| {
                (
                    transform.translation - rigid_body_position.0,
                    transform.rotation,
                    collider.clone(),
                )
            })
        })
        .collect::<Vec<_>>();

    (!colliders.is_empty()).then(|| Collider::compound(colliders))
}

pub(crate) trait GetBestGlobalTransform {
    fn get_best_global_transform(&self, entity: Entity) -> Transform;
}

impl GetBestGlobalTransform
    for Query<'_, '_, (&GlobalTransform, Option<&Position>, Option<&Rotation>)>
{
    fn get_best_global_transform(&self, entity: Entity) -> Transform {
        let (global_transform, position, rotation) = self
            .get(entity)
            .expect("Got an entity without `GlobalTransform`");
        if let Some(position) = position {
            if let Some(rotation) = rotation {
                return Transform::from_translation(position.0).with_rotation(rotation.0);
            }
        }
        global_transform.compute_transform()
    }
}
