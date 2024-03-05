use std::ops::Not;

use crate::state::{GameState, Screen};
use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
enum ScoreMenuButton {
    Continue,
    RageQuit,
}

impl Not for ScoreMenuButton {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Continue => Self::RageQuit,
            Self::RageQuit => Self::Continue,
        }
    }
}

#[derive(Component)]
struct ButtonMarker;

#[derive(Resource)]
struct SelectedButton(Option<ScoreMenuButton>, Interaction);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// used to show which player scored the touch
pub struct ScoreScreenPlugin;

impl Plugin for ScoreScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TouchScored), spawn_buttons)
            .add_systems(OnExit(Screen::TouchScored), despawn_buttons)
            .add_systems(
                Update,
                keyboard_select.run_if(in_state(Screen::TouchScored)),
            )
            .add_systems(Update, mouse_select.run_if(in_state(Screen::TouchScored)))
            .add_systems(
                Update,
                button_selection.run_if(in_state(Screen::TouchScored)),
            );
    }
}

/// spawns the buttons (and button text) of the pause menu
fn spawn_buttons(mut commands: Commands, world_state: Res<GameState>) {
    info!("spawning pause menu buttons");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ButtonMarker,
        ))
        // spawn player annoucement
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Player {:?}, scored!", world_state.row.unwrap()),
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));
        })
        // spawn score board
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!(
                    "Player One => {:<2} | Player Two => {:<2}",
                    world_state.p1_score.touches, world_state.p2_score.touches
                ),
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));
        })
        // spawn Continue Button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Vw(10.0),
                        height: Val::Vh(5.0),
                        border: UiRect::all(Val::Px(1.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    // transform: Transform::from_xyz(
                    //     window.width() / 2.0,
                    //     (window.height() / 4.0) * 2.0,
                    //     0.0,
                    // ),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Next Bout",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ScoreMenuButton::Continue,
                    ));
                });
        })
        // Rage Quit button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Vw(10.0),
                        height: Val::Vh(5.0),
                        border: UiRect::all(Val::Px(1.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    // transform: Transform::from_xyz(
                    //     window.width() / 2.0,
                    //     (window.height() / 4.0) * 1.0,
                    //     0.0,
                    // ),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Rage Quit",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ScoreMenuButton::RageQuit,
                    ));
                });
        });

    commands.insert_resource(SelectedButton(None, Interaction::None));
}

/// despwans pause menu icons
fn despawn_buttons(mut commands: Commands, buttons_query: Query<Entity, With<ButtonMarker>>) {
    buttons_query
        .iter()
        .for_each(|button| commands.entity(button).despawn_recursive());
}

/// handles using the keyboard to select a button
fn keyboard_select(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut button_selection: ResMut<SelectedButton>,
) {
    match button_selection.0 {
        Some(button) => {
            if keyboard_input.just_released(KeyCode::ArrowUp)
                || keyboard_input.just_released(KeyCode::ArrowDown)
                || keyboard_input.just_released(KeyCode::Tab)
            {
                debug!("changing button selection with keeb");
                button_selection.0 = Some(!button);
            } else if keyboard_input.just_released(KeyCode::Enter) {
                debug!("pressing button with keeb");
                button_selection.1 = Interaction::Pressed;
            }
        }
        None => {
            if keyboard_input.just_released(KeyCode::ArrowUp)
                || keyboard_input.just_released(KeyCode::ArrowDown)
                || keyboard_input.just_released(KeyCode::Tab)
            {
                debug!("selecting continue button with keeb");
                button_selection.0 = Some(ScoreMenuButton::Continue);
                button_selection.1 = Interaction::Hovered;
            }
        }
    }
}

/// handles using the mouse to select a button
fn mouse_select(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&ScoreMenuButton, With<Text>>,
    mut button_selection: ResMut<SelectedButton>,
) {
    for (interaction, children) in &mut interaction_query {
        let button_type = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed | Interaction::Hovered => {
                debug!("selecting with mouse");
                *button_selection = SelectedButton(Some(button_type.clone()), *interaction);
            }
            Interaction::None => {
                debug!("unselecting with mouse");
                *button_selection = SelectedButton(None, *interaction);
            }
        }
    }
}

/// handles changing the selected buttons collor and text
fn button_selection(
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), With<Button>>,
    mut text_query: Query<(&mut Text, &ScoreMenuButton)>,
    mut next_state: ResMut<NextState<Screen>>,
    button_selection: Res<SelectedButton>,
) {
    let select_prefix = "> ";

    for (mut color, mut border_color, children) in &mut button_query {
        let (mut text, button_type) = text_query.get_mut(children[0]).unwrap();

        if Some(*button_type) == button_selection.0 {
            match button_selection.1 {
                Interaction::Pressed => {
                    text.sections[0].value = text.sections[0]
                        .value
                        .strip_prefix(select_prefix)
                        .unwrap_or(text.sections[0].value.as_str())
                        .to_string();
                    *color = PRESSED_BUTTON.into();
                    next_state.set(press_button(button_type.clone()));
                }
                Interaction::Hovered => {
                    if !text.sections[0].value.starts_with(select_prefix) {
                        text.sections[0].value =
                            format!("{select_prefix}{}", text.sections[0].value);
                    }

                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    text.sections[0].value = text.sections[0]
                        .value
                        .strip_prefix(select_prefix)
                        .unwrap_or(text.sections[0].value.as_str())
                        .to_string();
                    *color = NORMAL_BUTTON.into();
                }
            }
        } else {
            text.sections[0].value = text.sections[0]
                .value
                .strip_prefix(select_prefix)
                .unwrap_or(text.sections[0].value.as_str())
                .to_string();
            *color = NORMAL_BUTTON.into();
            border_color.0 = Color::BLACK;
        }
    }
}

/// called by mouse select or keyboard select. used to envoke the buttons function
fn press_button(button: ScoreMenuButton) -> Screen {
    match button {
        ScoreMenuButton::Continue => {
            info!("starting a new bout");
            Screen::NewBout
        }
        _ => {
            // error!("the {:?} button has not been programed yet", button);
            Screen::ExitGame
        }
    }
}
