use std::iter;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(query.in_set(AvianPickupSystem::SpatialQuery));
}

/// Adapted from <https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690>
fn query(
    mut r_pickup: EventReader<AvianPickupEvent>,
    spatial_query: SpatialQuery,
    q_actor: Query<(&GlobalTransform, &AvianPickupActor, &AssociatedColliders)>,
    q_collider: Query<&ColliderParent>,
    q_rigid_body: Query<(&RigidBody, &GlobalTransform)>,
) {
    for event in r_pickup.read() {
        if !matches!(event, AvianPickupEvent::TryPickup) {
            continue;
        }
        for (origin, config, actor_colliders) in q_actor.iter() {
            let origin = origin.compute_transform();
            let query_filter = config
                .spatial_query_filter
                .clone()
                .with_excluded_entities(actor_colliders.0.clone());

            let mut nearest_dist = config.trace_length + 1.0;
            let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();

            let colliders = spatial_query.shape_intersections(
                &box_collider,
                origin.translation,
                origin.rotation,
                query_filter.clone(),
            );
            let mut nearest_entity = None;

            for collider in colliders {
                let rigid_body_entity = q_collider
                    .get(collider)
                    .expect("`shape_intersections` returned something without a `Collider`")
                    .get();
                let (&rigid_body, object_transform) = q_rigid_body
                    .get(rigid_body_entity)
                    .expect("Failed to get `RigidBody` for entity");
                if rigid_body != RigidBody::Dynamic {
                    continue;
                }
                let object_translation = object_transform.translation();

                // Closer than other objects
                let los = object_translation - origin.translation;
                let (los, dist) =
                    Dir3::new_and_length(los).expect("Failed to normalize line of sight");
                if dist >= nearest_dist {
                    continue;
                }

                // Cull to the cone
                let max_dot = config.cone;
                if los.dot(origin.forward().into()) <= max_dot {
                    continue;
                }

                // Make sure it isn't occluded!
                if let Some(hit) = spatial_query.cast_ray(
                    origin.translation,
                    los,
                    dist,
                    true,
                    query_filter.clone(),
                ) {
                    if hit.entity == rigid_body_entity {
                        nearest_dist = dist;
                        nearest_entity.replace(rigid_body_entity);
                    }
                }
            }
            info!("Nearest entity: {:?}", nearest_entity)
        }
    }
}
