use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(query.in_set(AvianPickupSystem::SpatialQuery));
}

/// Adapted from <https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/server/hl2/weapon_physcannon.cpp#L2690>
fn query(mut r_pickup: EventReader<AvianPickupEvent>, spatial_query: SpatialQuery, config: Res<AvianPickupConfig>) {
    for event in r_pickup.read() {
        if !matches!(event, AvianPickupEvent::TryPickup) {
            continue;
        }
        let box_shape = Cuboid::from_size(Vec3::splat(0.5));
        //spatial_query.shape_intersections()
    }
}
