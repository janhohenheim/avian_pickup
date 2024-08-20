use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use avian_pickup::prelude::*;
use bevy::{color::palettes::tailwind, input::mouse::MouseMotion, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_transform_interpolation::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            AvianPickupPlugin::default(),
            TransformInterpolationPlugin {
                global_translation_interpolation: true,
                global_rotation_interpolation: true,
                global_scale_interpolation: true,
            },
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
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(-Vec3::Z, Vec3::Y),
            ..default()
        },
        AvianPickupActor {
            prop_filter: SpatialQueryFilter::from_mask(ColliderLayer::Prop),
            terrain_filter: SpatialQueryFilter::from_mask(ColliderLayer::Terrain),
            ..default()
        },
        RotateCamera::default(),
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

fn rotate_camera(time: Res<Time>, mut camera: Query<(&mut Transform, &RotateCamera)>) {
    let dt = time.delta_seconds();
    let x_sensitive = 0.08;
    let y_sensitive = 0.05;
    for (mut transform, rotate) in camera.iter_mut() {
        let motion = rotate.0;
        // The factors are just arbitrary mouse sensitivity values.
        let delta_yaw = -motion.x * dt * x_sensitive;
        let delta_pitch = -motion.y * dt * y_sensitive;

        // Add yaw
        transform.rotate_y(delta_yaw);

        // Add pitch
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn debug(q_state: Query<&AvianPickupActorState, Changed<AvianPickupActorState>>) {
    for state in q_state.iter() {
        info!("{state:?}");
    }
}
