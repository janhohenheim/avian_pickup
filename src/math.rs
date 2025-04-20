use avian3d::prelude::*;
use bevy::prelude::*;

pub(crate) const METERS_PER_INCH: f32 = 0.0254;
pub(crate) fn rigid_body_compound_collider(
    colliders: Option<&[Entity]>,
    q_collider: &Query<(&GlobalTransform, &Collider, Option<&CollisionLayers>)>,
    filter: &SpatialQueryFilter,
) -> Option<Collider> {
    let collider_entities = colliders?;
    let mut colliders = Vec::new();
    for &entity in collider_entities {
        let (transform, collider, layers) = q_collider.get(entity).ok()?;
        let transform = transform.compute_transform();
        let layers = layers.copied().unwrap_or_default();
        if filter.test(entity, layers) {
            if let Some(compound) = collider.shape_scaled().as_compound() {
                // Need to unpack compound shapes because we are later returning a big compound collider for the whole rigid body
                // and parry crashes on nested compound shapes
                for (isometry, shape) in compound.shapes() {
                    let translation = Vec3::from(isometry.translation);
                    let rotation = Quat::from(isometry.rotation);
                    colliders.push((
                        transform.translation + translation,
                        transform.rotation * rotation,
                        shape.clone().into(),
                    ));
                }
            } else {
                colliders.push((transform.translation, transform.rotation, collider.clone()));
            }
        }
    }

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
