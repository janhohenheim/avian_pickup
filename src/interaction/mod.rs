//! Module for interation
use bevy_app::prelude::*;

mod finder;
mod hold;
mod pull;
mod push;

pub use self::{
    finder::{FindProp, PropFinder},
    hold::prelude::{HeldBy, Holding},
    pull::{PropPulled, PullRequest},
    push::{PropPushed, PushRequest},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hold::plugin, pull::plugin, push::plugin));
}

/// Prelude for every interation
pub mod prelude {
    pub(crate) use super::hold::prelude::*;
    pub use super::{
        FindProp, HeldBy, Holding, PropFinder, PropPulled, PropPushed, PullRequest, PushRequest,
    };
}
