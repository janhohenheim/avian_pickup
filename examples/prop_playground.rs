//! Shows a minimal example of using `avian_pickup` with Bevy.
//! This one contains a lot of props to play around with.

use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use avian_pickup::{prelude::*, prop::PreferredPickupDistanceOverride};
use bevy::{
    app::RunFixedMainLoop, color::palettes::tailwind, input::mouse::MouseMotion, prelude::*,
    time::run_fixed_main_schedule,
};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            // Because we are moving the camera independently of the physics system,
            // interpolation is needed to prevent jittering.
            AvianInterpolationPlugin::default(),
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
            (handle_input, rotate_camera).before(run_fixed_main_schedule),
        )
        .run();
}

/// Spawn the camera, light, ground, and a box to pick up.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let cube_material = materials.add(Color::from(tailwind::EMERALD_300));
    let plank_material = materials.add(Color::from(tailwind::TEAL_300));
    let ball_material = materials.add(Color::from(tailwind::ORANGE_300));
    let cylinder_material = materials.add(Color::from(tailwind::FUCHSIA_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0),
            ..default()
        },
        // Add this to set up the camera as the entity that can pick up
        // objects.
        AvianPickupActor {
            interaction_distance: 20.0,
            throw: AvianPickupActorThrowConfig {
                linear_speed_range: 0.0..=8.0,
                ..default()
            },
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
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));

    let ground_shape = Cuboid::new(15.0, 0.25, 15.0);
    let ground_mesh = meshes.add(Mesh::from(ground_shape));
    let terrain_transforms = [
        Transform::default(),
        Transform::from_xyz(7.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        Transform::from_xyz(-7.5, 0.0, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        Transform::from_xyz(0.0, 0.0, 7.5).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        Transform::from_xyz(0.0, 0.0, -7.5).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ];
    for (i, transform) in terrain_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Wall {}", i)),
            PbrBundle {
                mesh: ground_mesh.clone(),
                material: terrain_material.clone(),
                transform: *transform,
                ..default()
            },
            RigidBody::Static,
            Collider::from(ground_shape),
        ));
    }

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    let box_mesh = meshes.add(box_shape);
    commands.spawn((
        Name::new("Light Box"),
        PbrBundle {
            mesh: box_mesh.clone(),
            material: cube_material.clone(),
            transform: Transform::from_xyz(0.0, 2.0, 3.5),
            ..default()
        },
        // All `RigidBody::Dynamic` entities are able to be picked up.
        RigidBody::Dynamic,
        Collider::from(box_shape),
    ));
    commands.spawn((
        Name::new("Medium Box"),
        PbrBundle {
            mesh: box_mesh.clone(),
            material: cube_material.clone(),
            transform: Transform::from_xyz(2.0, 2.0, 2.0).with_scale(Vec3::splat(1.75)),
            ..default()
        },
        // All `RigidBody::Dynamic` entities are able to be picked up.
        RigidBody::Dynamic,
        Collider::from(box_shape),
        ColliderDensity(10.0),
    ));
    commands.spawn((
        Name::new("Heavy Box"),
        PbrBundle {
            mesh: box_mesh.clone(),
            material: cube_material.clone(),
            transform: Transform::from_xyz(-2.0, 2.0, 2.0).with_scale(Vec3::splat(2.5)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::from(box_shape),
        ColliderDensity(15.0),
        PreferredPickupDistanceOverride(2.5),
    ));

    let plan_transforms = [
        Transform::from_xyz(0.0, 4.0, 2.0).with_scale(Vec3::new(7.5, 0.2, 1.5)),
        Transform::from_xyz(0.0, 4.5, 2.0).with_scale(Vec3::new(7.5, 0.2, 1.5)),
        Transform::from_xyz(0.0, 5.0, 2.0).with_scale(Vec3::new(7.5, 0.2, 1.5)),
    ];
    for (i, transform) in plan_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Plank {i}")),
            PbrBundle {
                mesh: box_mesh.clone(),
                material: plank_material.clone(),
                transform: *transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(box_shape),
            ColliderDensity(11.0),
            PreferredPickupDistanceOverride(2.5),
        ));
    }

    let ball_shape = Sphere::new(0.5);
    let ball_mesh = meshes.add(Mesh::from(ball_shape));
    let ball_transforms = [
        Transform::from_xyz(6.0, 2.0, -3.0),
        Transform::from_xyz(6.0, 3.0, -4.0).with_scale(Vec3::splat(1.5)),
        Transform::from_xyz(6.0, 2.0, -5.0).with_scale(Vec3::splat(0.5)),
        Transform::from_xyz(6.0, 2.0, -6.0).with_scale(Vec3::splat(0.5)),
    ];
    for (i, transform) in ball_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Ball {i}")),
            PbrBundle {
                mesh: ball_mesh.clone(),
                material: ball_material.clone(),
                transform: *transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(ball_shape),
            ColliderDensity(9.0),
        ));
    }

    let cylinder_shape = Cylinder::new(0.145, 0.5);
    let cylinder_mesh = meshes.add(Mesh::from(cylinder_shape));

    // stack a pyramid of 6 cylinders
    let cylinder_transforms = [
        Transform::from_xyz(-1.0, 0.5, -1.0),
        Transform::from_xyz(-0.7, 0.5, -1.0),
        Transform::from_xyz(-0.4, 0.5, -1.0),
        Transform::from_xyz(-0.85, 1.1, -1.0),
        Transform::from_xyz(-0.55, 1.1, -1.0),
        Transform::from_xyz(-0.7, 1.65, -1.0),
    ];

    for (i, transform) in cylinder_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Cylinder {i}")),
            PbrBundle {
                mesh: cylinder_mesh.clone(),
                material: cylinder_material.clone(),
                transform: *transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(cylinder_shape),
            ColliderDensity(8.0),
        ));
    }
}

/// Pass player input along to `avian_pickup`
fn handle_input(
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    for actor in &actors {
        if key_input.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.send(AvianPickupInput {
                action: AvianPickupAction::Throw,
                actor,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                action: AvianPickupAction::Drop,
                actor,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                action: AvianPickupAction::Pull,
                actor,
            });
        }
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut cameras {
        let dt = time.delta_seconds();
        // The factors are just arbitrary mouse sensitivity values.
        // It's often nicer to have a faster horizontal sensitivity than vertical.
        let mouse_sensitivity = Vec2::new(0.12, 0.10);

        for motion in mouse_motion.read() {
            let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw + delta_yaw;
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}
