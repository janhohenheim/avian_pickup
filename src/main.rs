use avian3d::prelude::*;
use avian_pickup::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            AvianPickupPlugin::default(),
        ))
        .run();
}
