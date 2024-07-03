use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.get_schedule_mut(PhysicsSchedule)
        .unwrap()
        .add_systems(query.in_set(AvianPickupSystem::SpatialQuery));
}

fn query(mut r_pickup: EventReader<AvianPickupEvent>) {
    for event in r_pickup.read() {
        match event {
            AvianPickupEvent::TryPickup => {
                info!("TryPickup");
            }
            AvianPickupEvent::StopPickup => {
                info!("StopPickup");
            }
            AvianPickupEvent::TogglePickup => {
                info!("TogglePickup");
            }
            AvianPickupEvent::ThrowHeldObject => {
                info!("ThrowHeldObject");
            }
        }
    }
}
