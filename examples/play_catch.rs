use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use avian3d::prelude::*;
use avian_pickup::prelude::*;
use bevy::{
    app::RunFixedMainLoop,
    color::palettes::tailwind,
    input::mouse::MouseMotion,
    prelude::*,
    time::run_fixed_main_schedule,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_transform_interpolation::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
            TransformInterpolationPlugin::interpolate_all(),
            AvianPickupPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            RunFixedMainLoop,
            (handle_input, rotate_camera).before(run_fixed_main_schedule),
        )
        .add_systems(
            PhysicsSchedule,
            (
                tick_timer,
                rotate_npc,
                make_npc_catch,
                on_npc_hold,
                on_player_throw,
                on_aim_timer,
                on_catch_timer,
            )
                .chain()
                .in_set(AvianPickupSystem::First),
        )
        .run();
}

#[derive(Debug, Component)]
struct Prop;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Clone, Default, Component)]
struct Npc {
    state: NpcState,
    timer: Timer,
}

#[derive(Debug, Clone, Copy, Default, Component)]
enum NpcState {
    #[default]
    Waiting,
    Catching,
    Aiming(Vec3),
}

impl Npc {
    const AIM_DURATION: f32 = 1.0;
    const MAX_CATCH_DURATION: f32 = 3.0;

    fn waiting(&mut self) {
        self.state = NpcState::Waiting;
        self.timer = Timer::default();
    }

    fn aiming_to(&mut self, dir: Vec3) {
        self.state = NpcState::Aiming(dir);
        self.timer = Timer::from_seconds(Self::AIM_DURATION, TimerMode::Once);
    }

    fn catching(&mut self) {
        self.state = NpcState::Catching;
        self.timer = Timer::from_seconds(Self::MAX_CATCH_DURATION, TimerMode::Once);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_material = materials.add(Color::WHITE);
    let npc_material = materials.add(Color::from(tailwind::LIME_300));
    let visor_material = materials.add(Color::from(tailwind::LIME_600));
    let prop_material = materials.add(Color::from(tailwind::ORANGE_300));

    commands.spawn((
        Name::new("Player Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0),
            ..default()
        },
        AvianPickupActor::default(),
        NoRotationInterpolation,
        Player,
    ));

    let npc_shape = Sphere::new(0.7);
    let visor_shape = Cuboid::from_size(Vec3::new(1.0, 0.5, 0.01));
    commands
        .spawn((
            Name::new("NPC"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(npc_shape)),
                material: npc_material.clone(),
                transform: Transform::from_xyz(0.0, 1.0, -5.0).looking_to(Vec3::Z, Vec3::Y),
                ..default()
            },
            AvianPickupActor::default(),
            Npc::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Visor"),
                PbrBundle {
                    mesh: meshes.add(Mesh::from(visor_shape)),
                    material: visor_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -0.71),
                    ..default()
                },
            ));
        });

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
        RigidBody::Dynamic,
        Collider::from(box_shape),
        Prop,
    ));
}

fn handle_input(
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    players: Query<Entity, (With<AvianPickupActor>, With<Player>)>,
) {
    for player in &players {
        if key_input.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.send(AvianPickupInput {
                kind: AvianPickupInputKind::JustPressedL,
                actor: player,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                kind: AvianPickupInputKind::JustPressedR,
                actor: player,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.send(AvianPickupInput {
                kind: AvianPickupInputKind::PressedR,
                actor: player,
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
        let mouse_sensitivity = Vec2::new(0.12, 0.10);

        for motion in mouse_motion.read() {
            let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

            transform.rotate_y(delta_yaw);

            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

fn rotate_npc(
    time: Res<Time>,
    mut npcs: Query<(&mut Transform, &Npc)>,
    props: Query<&Transform, (With<Prop>, Without<Npc>)>,
) {
    let Ok(prop) = props.get_single() else {
        return;
    };
    let dt = time.delta_seconds();

    for (mut transform, npc) in &mut npcs {
        let dir = match npc.state {
            NpcState::Waiting | NpcState::Catching => prop.translation - transform.translation,
            NpcState::Aiming(dir) => dir,
        };
        let target = transform.looking_to(dir, Vec3::Y);
        let decay_rate = f32::ln(10.0);
        transform.rotation = transform
            .rotation
            .slerp(target.rotation, 1.0 - f32::exp(-decay_rate * dt));
    }
}

fn tick_timer(time: Res<Time>, mut npcs: Query<&mut Npc>) {
    for mut npc in &mut npcs {
        npc.timer.tick(time.delta());
    }
}

fn make_npc_catch(
    mut npcs: Query<(Entity, &Npc)>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    for (entity, npc) in &mut npcs {
        if !matches!(npc.state, NpcState::Catching) {
            continue;
        }
        avian_pickup_input_writer.send(AvianPickupInput {
            kind: AvianPickupInputKind::PressedR,
            actor: entity,
        });
    }
}

fn on_npc_hold(
    mut npcs: Query<(&mut Npc, &AvianPickupActorState), Changed<AvianPickupActorState>>,
) {
    for (mut npc, state) in &mut npcs {
        if !matches!(state, AvianPickupActorState::Holding(..)) {
            continue;
        }
        let mut rng = rand::thread_rng();
        let max_pitch = FRAC_PI_4;
        let max_yaw = FRAC_PI_4;
        let random_pitch = rng.gen_range(-max_pitch..max_pitch);
        let random_yaw = rng.gen_range(-max_yaw..max_yaw);
        let rotation = Quat::from_euler(EulerRot::YXZ, random_yaw, random_pitch, 0.0);
        let dir = rotation.mul_vec3(Vec3::Z);
        npc.aiming_to(dir);
    }
}

fn on_aim_timer(
    mut npcs: Query<(Entity, &mut Npc)>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    for (entity, mut npc) in &mut npcs {
        if !matches!(npc.state, NpcState::Aiming(..)) || !npc.timer.finished() {
            continue;
        }
        npc.waiting();
        avian_pickup_input_writer.send(AvianPickupInput {
            kind: AvianPickupInputKind::JustPressedL,
            actor: entity,
        });
    }
}

fn on_catch_timer(mut npcs: Query<&mut Npc>, mut props: Query<&mut Transform, With<Prop>>) {
    for mut npc in &mut npcs {
        if !matches!(npc.state, NpcState::Catching) || !npc.timer.finished() {
            continue;
        }
        npc.waiting();
        for mut prop in &mut props {
            prop.translation = Vec3::new(0.0, 2.0, 0.0);
        }
    }
}

fn on_player_throw(
    mut throw_events: EventReader<PropThrown>,
    mut npcs: Query<&mut Npc>,
    players: Query<(), With<Player>>,
) {
    for event in throw_events.read() {
        if players.contains(event.actor) {
            for mut npc in &mut npcs {
                npc.catching();
            }
        }
    }
}
