//! TODO

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(PropThrown, PropDropped)>()
        .add_event::<PropThrown>()
        .add_event::<PropDropped>();
}

pub(super) mod prelude {
    pub use super::{PropDropped, PropThrown};
}

/// Event sent when a prop is thrown by an actor.
/// This is meant for the user to lister to in order to play sound effects, etc.
/// Sending this has no effect on the prop itself.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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
    /// Whether the prop was held when thrown. If `false`, the prop was directly
    /// thrown while an actor was looking at it without holding it up first.
    pub was_held: bool,
}

/// Event sent when a prop is dropped by an actor.
/// This is meant for the user to lister to in order to play sound effects, etc.
/// Sending this has no effect on the prop itself.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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
