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

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PropThrown {
    pub prop: Entity,
    pub actor: Entity,
    pub was_held: bool,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PropDropped {
    pub prop: Entity,
    pub actor: Entity,
    pub forced: bool,
}
