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
            PhysicsDebugPlugin::default(),
            AvianPickupPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (read_pickup_input, read_camera_input))
        .add_systems(
            PhysicsSchedule,
            rotate_camera.in_set(AvianPickupSystem::First),
        )
        .add_systems(PhysicsSchedule, debug.in_set(AvianPickupSystem::Last))
        .run();
}

#[derive(Debug, PhysicsLayer)]
enum ColliderLayer {
    Default,
    Player,
    Prop,
    Terrain,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let static_material = materials.add(Color::WHITE);
    let dynamic_material = materials.add(Color::from(tailwind::EMERALD_300));

    commands.spawn((
        Name::new("Player Collider"),
        RigidBody::Kinematic,
        SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 1.0, 5.0).looking_at(-Vec3::Z, Vec3::Y),
        ),
        Collider::capsule(0.3, 1.2),
        CollisionLayers::new(
            ColliderLayer::Player,
            [
                ColliderLayer::Default,
                ColliderLayer::Prop,
                ColliderLayer::Terrain,
            ],
        ),
    ));
    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.2, 5.0).looking_at(-Vec3::Z, Vec3::Y),
            ..default()
        },
        AvianPickupActor {
            prop_filter: SpatialQueryFilter::from_mask(ColliderLayer::Prop),
            obstacle_filter: SpatialQueryFilter::from_mask(ColliderLayer::Terrain),
            actor_filter: SpatialQueryFilter::from_mask(ColliderLayer::Player),
            // Make sure the props do not intersect with the player's capsule
            // when looking down.
            hold: AvianPickupActorHoldConfig {
                pitch_range: -50.0_f32.to_radians()..=75.0_f32.to_radians(),
                ..default()
            },
            ..default()
        },
        RigidBody::Kinematic,
        RotateCamera::default(),
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
        CollisionLayers::new(
            ColliderLayer::Terrain,
            [
                ColliderLayer::Default,
                ColliderLayer::Prop,
                ColliderLayer::Player,
            ],
        ),
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(0.5));
    commands.spawn((
        Name::new("Box"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(box_shape)),
            material: dynamic_material.clone(),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::from(box_shape),
        CollisionLayers::new(
            ColliderLayer::Prop,
            [
                ColliderLayer::Default,
                ColliderLayer::Terrain,
                ColliderLayer::Player,
            ],
        ),
    ));
}

fn read_pickup_input(
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    let Ok(actor) = actors.get_single() else {
        return;
    };
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

#[derive(Debug, Default, Component)]
struct RotateCamera(Vec2);

fn read_camera_input(
    mut mouse_motion: EventReader<MouseMotion>,
    mut rotate_camera: Query<&mut RotateCamera>,
) {
    for mut rotate in rotate_camera.iter_mut() {
        rotate.0 = Vec2::ZERO;
        for motion in mouse_motion.read() {
            rotate.0 += motion.delta;
        }
    }
}

fn rotate_camera(time: Res<Time>, mut camera: Query<(&mut Rotation, &RotateCamera)>) {
    let dt = time.delta_seconds();
    let mouse_sensitivity = Vec2::new(0.08, 0.05);
    for (mut rotation, motion) in camera.iter_mut() {
        let motion = motion.0;
        // The factors are just arbitrary mouse sensitivity values.
        let delta_yaw = -motion.x * dt * mouse_sensitivity.x;
        let delta_pitch = -motion.y * dt * mouse_sensitivity.y;

        // Add yaw (global rotation)
        rotation.0 = Quat::from_rotation_y(delta_yaw) * rotation.0;

        // Add pitch (local rotation)
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        rotation.0 = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn debug(q_state: Query<&AvianPickupActorState, Changed<AvianPickupActorState>>) {
    for state in q_state.iter() {
        info!("{state:?}");
    }
}
