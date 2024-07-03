use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupCamera>();
}

#[derive(Debug, Clone, Copy, Hash, Component, Default, PartialEq, Eq, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", reflect(Serialize, Deserialize))]
#[reflect(Debug, Component, Default, Hash, PartialEq)]
/// Tag component for the camera that will be used for picking up objects.
/// Place this on the camera entity that is under the player control.
///
/// # Example
/// ```
/// # use avian_pickup::prelude::*;
/// fn setup_camera(mut commands: Commands) {
///     commands.spawn((Name::new("Player Camera"), Camera3dBundle::default(), AvianPickupCamera));
/// }
/// ```
pub struct AvianPickupCamera;
