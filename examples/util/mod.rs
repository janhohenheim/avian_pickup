use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

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

/// Used to tell `spawn_text` which instructions to spawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    /// The minimal set of instructions.
    #[allow(dead_code)]
    Generic,
    /// Includes instructions for resetting the cube's position.
    #[allow(dead_code)]
    Resettable,
    /// Includes instructions for manipulating the cube's position and rotation.
    #[allow(dead_code)]
    Manipulation,
}

/// Show a crosshair for better aiming
fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_texture = asset_server.load("crosshair.png");
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageNode::new(crosshair_texture));
        });
}

/// Capture the cursor when the left mouse button is pressed.
/// This makes it way less fidgity to pick up objects.
fn capture_cursor(mut cursor_options: Query<&mut CursorOptions>) {
    for mut cursor_options in &mut cursor_options {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

/// Release the cursor when the escape key is pressed.
/// Somehow doesn't work on macOS?
fn release_cursor(mut cursor_options: Query<&mut CursorOptions>) {
    for mut cursor_options in &mut cursor_options {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

/// Spawn instructions for the user, depending on the example.
fn spawn_text(example: Example) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands
            .spawn(Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        Text::default(),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextSpan::new(match example {
                            Example::Generic => concat!(
                                "Move the camera with your mouse.\n",
                                "Hold right click to pick up a prop.\n",
                                "Left click while holding a prop to throw it.\n",
                                "Right click while holding a prop to drop it.\n",
                                "Press Escape to release the cursor."
                            ),
                            Example::Resettable => concat!(
                                "Move the camera with your mouse.\n",
                                "Hold right click to pick up a prop.\n",
                                "Left click while holding a prop to throw it.\n",
                                "Right click while holding a prop to drop it.\n",
                                "Press R to reset the prop's position.\n",
                                "Press Escape to release the cursor."
                            ),
                            Example::Manipulation => concat!(
                                "Move the camera with your mouse.\n",
                                "Hold right click to pick up a prop.\n",
                                "Left click while holding a prop to throw it.\n",
                                "Right click while holding a prop to drop it.\n",
                                "Scroll to change the held prop's distance.\n",
                                "Hold Shift and move the mouse to rotate the held prop.\n",
                                "Press Escape to release the cursor."
                            ),
                        }));
                    });
            });
    }
}
