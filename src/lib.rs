#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use bevy::prelude::*;

pub mod prelude {
    pub use crate::AvianPickupPlugin;
}

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct AvianPickupPlugin;

impl Plugin for AvianPickupPlugin {
    fn build(&self, app: &mut App) {}
}
