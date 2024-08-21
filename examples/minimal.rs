use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_pickup::prelude::*;
use bevy::{
    color::palettes::tailwind,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::Viewport,
    window::WindowResized,
};
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
        .add_systems(
            Update,
            (
                set_camera_viewports,
                toggle_choice,
                handle_input,
                rotate_camera,
            )
                .chain(),
        )
        .add_systems(PhysicsSchedule, debug.in_set(AvianPickupSystem::Last))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let static_material = materials.add(Color::WHITE);
    let dynamic_material = materials.add(Color::from(tailwind::EMERALD_300));

    commands.spawn((
        Name::new("Player Camera Rotation"),
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 1.0, 5.0).looking_to(-Vec3::Z, Vec3::Y),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        CameraPosition {
            pos: UVec2::new((0 % 2) as u32, (0 / 2) as u32),
        },
        AvianPickupActor::default(),
        RigidBody::Kinematic,
        Choice::Rotation,
    ));

    commands.spawn((
        Name::new("Player Camera Transform"),
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 1.0, 5.0).looking_to(-Vec3::Z, Vec3::Y),
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        CameraPosition {
            pos: UVec2::new((1 % 2) as u32, (1 / 2) as u32),
        },
        AvianPickupActor::default(),
        RigidBody::Kinematic,
        Choice::Transform,
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
            material: static_material.clone(),
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
            material: dynamic_material.clone(),
            transform: Transform::from_xyz(-5.0, 2.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::from(box_shape),
    ));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: dynamic_material.clone(),
            transform: Transform::from_xyz(5.0, 2.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::from(box_shape),
    ));
}

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

#[derive(Component)]
enum Choice {
    Rotation,
    Transform,
}

fn toggle_choice(key_input: Res<ButtonInput<KeyCode>>, mut camera: Query<&mut Choice>) {
    if key_input.just_pressed(KeyCode::Space) {
        for mut choice in camera.iter_mut() {
            *choice = match *choice {
                Choice::Rotation => Choice::Transform,
                Choice::Transform => Choice::Rotation,
            };
        }
    }
}

fn rotate_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera: Query<(&mut Rotation, &mut Transform, &Choice)>,
) {
    for motion in mouse_motion.read() {
        for (mut rotation, mut transform, choice) in camera.iter_mut() {
            let rotation = match choice {
                Choice::Rotation => &mut rotation.0,
                Choice::Transform => &mut transform.rotation,
            };
            // The factors are just arbitrary mouse sensitivity values.
            let delta_yaw = -motion.delta.x * 0.003;
            let delta_pitch = -motion.delta.y * 0.002;

            // Add yaw
            *rotation = Quat::from_rotation_y(delta_yaw) * *rotation;

            // Add pitch
            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            *rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn debug(q_state: Query<&AvianPickupActorState, Changed<AvianPickupActorState>>) {
    for state in q_state.iter() {
        info!("{state:?}");
    }
    println!("");
}

#[derive(Component)]
struct CameraPosition {
    pos: UVec2,
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraPosition, &mut Camera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size
    // changes so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse
    // this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = window.physical_size() / 2;

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}
