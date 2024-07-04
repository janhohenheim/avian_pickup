use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AvianPickupCamera>();
}

/// Tag component for the camera that will be used for picking up objects.
/// Place this on the camera entity that is under the player control.
///
/// # Example
/// ```
/// # use avian_pickup::prelude::*;
/// # use bevy::prelude::*;
///
/// fn setup_camera(mut commands: Commands) {
///     commands.spawn((
///         Name::new("Player Camera"),
///         Camera3dBundle::default(),
///         AvianPickupCamera,
///     ));
/// }
/// ```
#[derive(Debug, Clone, Copy, Hash, Component, Default, PartialEq, Eq, Reflect)]
#[reflect(Debug, Component, Default, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct AvianPickupCamera;
