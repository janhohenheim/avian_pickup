use avian3d::{math::TAU, prelude::*};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, push_lid)
        .run();
}

#[derive(Debug, Component)]
struct Lid;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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

    let body_shape = Cuboid::new(1.0, 1.0, 1.0);
    let lid_shape = Cuboid::new(1.0, 0.1, 1.0);

    let body_mesh = meshes.add(Mesh::from(body_shape));
    let lid_mesh = meshes.add(Mesh::from(lid_shape));

    let y_offset = 0.1;
    let body_joint_anchor = Vec3::new(-0.5, 0.5 + y_offset, 0.0);
    let lid_joint_anchor = Vec3::new(-0.5, -0.05 - y_offset, 0.0);

    let body = commands
        .spawn((
            Name::new("Body"),
            PbrBundle {
                mesh: body_mesh,
                material: white.clone(),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            RigidBody::Static,
            Collider::from(body_shape),
        ))
        .id();

    let mut lid_transform = Transform::from_xyz(0.0, 2.2, 0.0);
    lid_transform.rotate_axis(Dir3::Z, TAU / 8.0);
    let lid = commands
        .spawn((
            Name::new("Lid"),
            PbrBundle {
                mesh: lid_mesh,
                material: white.clone(),
                transform: lid_transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::from(lid_shape),
            Lid,
        ))
        .id();

    commands.spawn((
        Name::new("RevoluteJoint"),
        RevoluteJoint::new(body, lid)
            .with_aligned_axis(Vec3::Z)
            .with_local_anchor_1(body_joint_anchor)
            .with_local_anchor_2(lid_joint_anchor),
    ));
}

fn push_lid(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut q_lid: Query<(&Transform, &mut ExternalImpulse), With<Lid>>,
) {
    let Ok((transform, mut impulse)) = q_lid.get_single_mut() else {
        return;
    };

    if mouse_button_input.just_pressed(MouseButton::Left) {
        impulse.apply_impulse(transform.rotation * Vec3::Y * 0.3);
    }
}
