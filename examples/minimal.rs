use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_pickup::prelude::*;
use bevy::{color::palettes::tailwind, input::mouse::MouseMotion, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            AvianPickupPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, rotate_camera))
        .add_systems(PhysicsSchedule, debug.in_set(AvianPickupSystem::Last))
        .run();
}

/// Spawn the camera, light, ground, and a box to pick up.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let prop_material = materials.add(Color::from(tailwind::EMERALD_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(-Vec3::Z, Vec3::Y),
            ..default()
        },
        // Add this to set up the camera as the entity that can pick up
        // objects.
        AvianPickupActor::default(),
        // Add a `RigidBody` so that `rotate_camera` can use `Rotation`.
        RigidBody::Kinematic,
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
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(ground_shape)),
            material: terrain_material.clone(),
            ..default()
        },
        RigidBody::Static,
        Collider::from(ground_shape),
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: prop_material.clone(),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        // All `RigidBody::Dynamic` entities are able to be picked up.
        RigidBody::Dynamic,
        Collider::from(box_shape),
    ));
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
                kind: AvianPickupInputKind::JustPressedL,
                actor,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                kind: AvianPickupInputKind::JustPressedR,
                actor,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                kind: AvianPickupInputKind::PressedR,
                actor,
            });
        }
    }
}

/// We change the `Rotation` and not the `Transform` because at this point,
/// Avian already ran using the previous frame's transform. This means that if
/// we update `Transform` now, the cube will lag one frame behind the camera.
fn rotate_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut cameras: Query<&mut Rotation, With<Camera>>,
) {
    for mut rotation in &mut cameras {
        let dt = time.delta_seconds();
        // The factors are just arbitrary mouse sensitivity values.
        // It's often nicer to have a faster horizontal sensitivity than vertical.
        let mouse_sensitivity = Vec2::new(0.08, 0.05);

        for motion in mouse_motion.read() {
            let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

            // Add yaw (global)
            rotation.0 = Quat::from_rotation_y(delta_yaw) * rotation.0;

            // Add pitch (local)
            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            rotation.0 = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn debug(q_state: Query<&AvianPickupActorState, Changed<AvianPickupActorState>>) {
    for state in q_state.iter() {
        info!("{state:?}");
    }
}
