//! Shows a minimal example of using `avian_pickup` with Bevy.

use std::f32::consts::FRAC_PI_2;

use avian_pickup::prelude::*;
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, prelude::*};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            AvianPickupPlugin::default(),
            // This is just here to make the example look a bit nicer.
            util::plugin(util::Example::Generic),
        ))
        .add_systems(Startup, setup)
        // Input handling and camera movement need to be executed every frame,
        // so we run them in a variable timestep.
        // We also want them to happen before the physics system, so we add them
        // to the last variable timestep schedule before the fixed timestep systems run.
        .add_systems(
            RunFixedMainLoop,
            (handle_input, rotate_camera).in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .run();
}

/// Spawn the actor, camera, light, ground, and a box to pick up.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let obstacle_material = materials.add(Color::from(tailwind::RED_300));
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 5.0),
        // Add this to set up the camera as the entity that can pick up
        // objects.
        AvianPickupActor {
            interaction_distance: 15.0,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        Transform::from_xyz(3.0, 8.0, 3.0),
        PointLight {
            color: Color::WHITE,
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
    ));

    let ground_shape = Cuboid::new(15.0, 0.25, 15.0);
    commands.spawn((
        Name::new("Ground"),
        Mesh3d::from(meshes.add(Mesh::from(ground_shape))),
        MeshMaterial3d::from(terrain_material.clone()),
        RigidBody::Static,
        Collider::from(ground_shape),
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        Mesh3d::from(meshes.add(Mesh::from(box_shape))),
        MeshMaterial3d::from(prop_material.clone()),
        Transform::from_xyz(0.0, 2.0, 3.5),
        // All `RigidBody::Dynamic` entities are able to be picked up.
        RigidBody::Dynamic,
        Collider::from(box_shape),
        // Because we are moving the camera independently of the physics system,
        // interpolation is needed to prevent jittering.
        TransformInterpolation,
    ));

    let column_shape = Cylinder::new(0.1, 2.0);
    commands.spawn((
        Name::new("Column"),
        Mesh3d::from(meshes.add(Mesh::from(column_shape))),
        MeshMaterial3d::from(obstacle_material.clone()),
        Transform::from_xyz(0.0, 1.0, 4.0),
        // As a static object, the column will not be picked up.
        RigidBody::Static,
        Collider::from(column_shape),
    ));
}

/// Pass player input along to `avian_pickup`
fn handle_input(
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    for actor in &actors {
        if key_input.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Throw,
                actor,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Drop,
                actor,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Pull,
                actor,
            });
        }
    }
}

fn rotate_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut transform: Single<&mut Transform, With<Camera>>,
) {
    // The factors are just arbitrary mouse sensitivity values.
    let camera_sensitivity = Vec2::new(0.001, 0.001);

    let delta = accumulated_mouse_motion.delta;
    let delta_yaw = -delta.x * camera_sensitivity.x;
    let delta_pitch = -delta.y * camera_sensitivity.y;

    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}
