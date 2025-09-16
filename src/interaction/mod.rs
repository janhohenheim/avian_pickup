use bevy_app::prelude::*;

mod drop;
mod hold;
mod pull;
mod throw;

pub(crate) use self::hold::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hold::plugin, pull::plugin, drop::plugin, throw::plugin));
}
