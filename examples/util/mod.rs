use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::CursorGrabMode};

pub fn plugin(example: Example) -> impl Plugin {
    move |app: &mut App| {
        app.add_systems(Startup, (spawn_crosshair, spawn_text(example)))
            // Purely aesthetic systems go in `Update`.
            .add_systems(
                Update,
                (
                    capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                    release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
                )
                    .chain(),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    Generic,
    Resettable,
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_texture = asset_server.load("crosshair.png");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: crosshair_texture.into(),
                ..default()
            });
        });
}

fn capture_cursor(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn release_cursor(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn spawn_text(example: Example) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    match example {
                        Example::Generic => concat!(
                            "Move the camera with your mouse.\n",
                            "Hold right click to pick up the cube.\n",
                            "Left click to throw it.\n",
                            "Press Escape to release the cursor."
                        ),
                        Example::Resettable => concat!(
                            "Move the camera with your mouse.\n",
                            "Hold right click to pick up the cube.\n",
                            "Left click to throw it.\n",
                            "Press Escape to release the cursor.\n",
                            "Press R to reset the cube's position."
                        ),
                    },
                    TextStyle {
                        font_size: 25.0,
                        ..default()
                    },
                ));
            });
    }
}
