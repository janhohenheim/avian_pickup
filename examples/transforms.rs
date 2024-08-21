use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, change_transform)
        .add_systems(
            PhysicsSchedule,
            read_rotations.in_set(PhysicsStepSet::First),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        RigidBody::Kinematic,
        ChangeTransform,
    ));
    commands.spawn((
        SpatialBundle::default(),
        RigidBody::Kinematic,
        ChangeRotation,
    ));
}

#[derive(Component)]
struct ChangeTransform;
#[derive(Component)]
struct ChangeRotation;

fn change_transform(
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_change_rotation: Query<&mut Rotation, With<ChangeRotation>>,
    mut q_change_transform: Query<&mut Transform, With<ChangeTransform>>,
) {
    let Ok(mut rotation) = q_change_rotation.get_single_mut() else {
        // Not available for the first 2 frames or so.
        return;
    };
    let mut transform = q_change_transform.single_mut();
    for motion in mouse_motion.read() {
        // The factors are just arbitrary mouse sensitivity values.
        let delta_yaw = -motion.delta.x * 0.003;
        let delta_pitch = -motion.delta.y * 0.002;

        // Add yaw
        rotation.0 = Quat::from_rotation_y(delta_yaw) * rotation.0;
        transform.rotation = Quat::from_rotation_y(delta_yaw) * transform.rotation;

        // Add pitch
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        rotation.0 = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn read_rotations(
    q_change_rotation: Query<&Rotation, With<ChangeRotation>>,
    q_change_transform: Query<&Rotation, With<ChangeTransform>>,
) {
    let rotation_from_rotation = q_change_rotation.single();
    let rotation_from_transform = q_change_transform.single();
    info!(
        "Rotation from Rotation: {:?}",
        rotation_from_rotation.to_euler(EulerRot::YXZ)
    );
    info!(
        "Rotation from Transform: {:?}",
        rotation_from_transform.to_euler(EulerRot::YXZ)
    );

    let delta_rotation = rotation_from_rotation.0 * rotation_from_transform.0.inverse();
    info!("Delta: {:?}", delta_rotation.to_euler(EulerRot::YXZ));

    assert!(delta_rotation.to_scaled_axis().length() < 0.001);

    println!("");
}
