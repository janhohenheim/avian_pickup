use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(query.in_set(AvianPickupSystem::SpatialQuery));
}

/// Adapted from <https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690>
fn query(
    mut r_pickup: EventReader<AvianPickupEvent>,
    q_camera: Query<(Entity, &GlobalTransform), With<AvianPickupCamera>>,
    spatial_query: SpatialQuery,
    config: Res<AvianPickupConfig>,
    q_collider: Query<Option<&ColliderParent>, With<Collider>>,
    q_rigid_body: Query<(&RigidBody, &GlobalTransform)>,
) {
    // TODO: This should maybe be in front of the camera?
    let (camera_entity, origin) = single!(q_camera);
    let origin = origin.compute_transform();

    for event in r_pickup.read() {
        if !matches!(event, AvianPickupEvent::TryPickup) {
            continue;
        }
        let mut nearest_dist = config.trace_length + 1.0;
        let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();
        // TODO: Allow the user to filter out certain entities and layers.
        let query_filter = SpatialQueryFilter::default().with_excluded_entities(camera_entity);

        let colliders = spatial_query.shape_intersections(
            &box_collider,
            origin.translation,
            origin.rotation,
            query_filter,
        );

        for collider in colliders {
            let rigid_body_entity = q_collider
                .get(collider)
                .expect("`shape_intersections` returned something without a `Collider`")
                .map_or(collider, ColliderParent::get);
            let (rigid_body, object_transform) = q_rigid_body
                .get(rigid_body_entity)
                .expect("Failed to get `RigidBody` for entity");
            if rigid_body != RigidBody::Dynamic {
                continue;
            }
            let object_transform = object_transform.compute_transform();

            // Closer than other objects
            let los = object_transform.translation - origin.translation;
            let (los, dist) = Dir3::new_and_length(los).expect("Failed to normalize line of sight");
            if dist >= nearest_dist {
                continue;
            }

            // Cull to the cone
            // TODO: Sometimes, punt_cone is used
            let max_dot = config.cone;
            if los.dot(origin.forward().into()) <= max_dot {
                continue;
            }

            // Make sure it isn't occluded!
            todo!();
        }
    }
}
