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
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let white = materials.add(Color::WHITE);

    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_to(-Vec3::Z, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 3.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2_000_000.0,
                ..default()
            },
            ..default()
        },
    ));

    let ground_shape = Cuboid::new(15.0, 0.25, 15.0);
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(ground_shape)),
            material: white.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::from(ground_shape),
    ));
}

fn handle_input(key_input: Res<ButtonInput<KeyCode>>) {
    if key_input.just_pressed(KeyCode::KeyE) {
        info!("E pressed!");
    }
}
