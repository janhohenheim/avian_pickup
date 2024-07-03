use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupEvent>().add_event::<AvianPickupEvent>();
}

/// Event for picking up and throwing objects.
/// Send this to tell Avian Pickup to do its thing.
#[derive(Event, Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize), reflect(Serialize, Deserialize))]
pub enum AvianPickupEvent {
    /// Try to pick up the object in front of the camera. If there is no object, do nothing.
    /// If there is, start applying a force to it until it is close enough to the camera to be picked up.
    /// Does nothing if there is already an object being held.
    TryPickup,
    /// Stop applying a force to an object that is being picked up or already held.
    StopPickup,
    /// Toggle the pickup state of the held object. Acts as [`AvianPickupEvent::StopPickup`] if we are trying to pick up an object or holding one, and
    /// as [`AvianPickupEvent::TryPickup`] otherwise.
    TogglePickup,
    /// Throw the held object. Does nothing if there is no object being held. This also does nothing if the object is still being picked up.
    ThrowHeldObject,
}
