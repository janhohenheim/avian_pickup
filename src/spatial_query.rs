use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(query.in_set(AvianPickupSystem::SpatialQuery));
}

/// Adapted from <https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690>
fn query(
    mut r_pickup: EventReader<AvianPickupEvent>,
    q_camera: Query<&GlobalTransform, With<AvianPickupCamera>>,
    spatial_query: SpatialQuery,
    config: Res<AvianPickupConfig>,
    q_collider: Query<Option<&ColliderParent>, With<Collider>>,
    q_rigid_body: Query<&RigidBody>,
) {
    let origin = single!(q_camera).compute_transform();
    for event in r_pickup.read() {
        if !matches!(event, AvianPickupEvent::TryPickup) {
            info!("Ignoring event: {:?}", event);
            continue;
        }
        let nearest_dist = config.trace_length + 1.0;
        let box_collider = Cuboid::from_size(Vec3::splat(2.0 * nearest_dist)).into();
        let query_filter = SpatialQueryFilter::default();

        let colliders = spatial_query.shape_intersections(&box_collider, origin.translation, origin.rotation, query_filter);
        let rigid_bodies = colliders
            .iter()
            .map(|&entity| {
                q_collider
                    .get(entity)
                    .expect("shape_intersections returned something without a collider")
                    .map_or(entity, ColliderParent::get)
            })
            .map(|entity| q_rigid_body.get(entity).expect("collider has no rigid body and no `ColliderParent`"));
        info!("rigid_bodies: {:?}", rigid_bodies);
    }
}
