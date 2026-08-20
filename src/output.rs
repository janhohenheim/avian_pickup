//! Events related to props being dropped.
//! These will be sent by the Avian Pickup plugin to notify the user of
//! prop-related events. Handle these to e.g. play sound effects or show
//! visual effects.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PropForcedDrop>();
}

pub(super) mod prelude {
    pub use super::PropForcedDrop;
}

/// Message sent when a prop is forced to be dropped by an actor.
/// A prop is forced to be drop by being too far away from its
/// target location.
/// Sending this has no effect on the prop itself.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PropForcedDrop {
    /// The dropped prop.
    pub prop: Entity,
    /// The actor that dropped the prop.
    pub actor: Entity,
}
