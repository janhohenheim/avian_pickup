//! Events related to props being thrown and dropped.
//! These will be sent by the Avian Pickup plugin to notify the user of
//! prop-related events. Handle these to e.g. play sound effects or show
//! visual effects.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<PropThrown>().add_message::<PropDropped>();
}

pub(super) mod prelude {
    pub use super::{PropDropped, PropThrown};
}

/// Message sent when a prop is thrown by an actor.
/// This is meant for the user to lister to in order to play sound effects, etc.
/// Sending this has no effect on the prop itself.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PropThrown {
    /// The thrown prop.
    pub prop: Entity,
    /// The actor that threw the prop.
    pub actor: Entity,
}

/// Message sent when a prop is dropped by an actor.
/// This is meant for the user to listen to in order to play sound effects, etc.
/// Sending this has no effect on the prop itself.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PropDropped {
    /// The dropped prop.
    pub prop: Entity,
    /// The actor that dropped the prop.
    pub actor: Entity,
    /// Whether the drop was forced to be dropped by being too far away from its
    /// target location. If `false`, the prop was dropped by the actor's own
    /// volition.
    pub forced: bool,
}
