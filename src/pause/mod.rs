use crate::state::Screen;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum PauseMenuButton {
    Continue,
    RageQuit,
    Contols,
}

impl PauseMenuButton {
    fn next(index: PauseMenuButton) -> Self {
        match index {
            Self::Continue => Self::RageQuit,
            Self::RageQuit => Self::Contols,
            Self::Contols => Self::Continue,
        }
    }

    fn prev(index: PauseMenuButton) -> Self {
        match index {
            Self::Continue => Self::Contols,
            Self::RageQuit => Self::Continue,
            Self::Contols => Self::RageQuit,
        }
    }
}

#[derive(Component)]
struct ButtonMarker;

#[derive(Resource)]
struct SelectedButton(Option<PauseMenuButton>, Interaction);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// used to show a pause menu
pub struct PauseScreenPlugin;

impl Plugin for PauseScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::PauseMenu), spawn_buttons)
            .add_systems(OnExit(Screen::PauseMenu), despawn_buttons)
            .add_systems(Update, keyboard_select.run_if(in_state(Screen::PauseMenu)))
            .add_systems(Update, mouse_select.run_if(in_state(Screen::PauseMenu)))
            .add_systems(Update, button_selection.run_if(in_state(Screen::PauseMenu)))
            .add_systems(Update, pause_game.run_if(in_state(Screen::Game)))
            .add_systems(Update, unpause_game.run_if(in_state(Screen::PauseMenu)));
    }
}

/// spawns the buttons (and button text) of the pause menu
fn spawn_buttons(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    info!("spawning pause menu buttons");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            },
            ButtonMarker,
        ))
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
                    transform: Transform::from_xyz(
                        window.width() / 2.0,
                        (window.height() / 5.0) * 4.0,
                        0.0,
                    ),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Continue",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        PauseMenuButton::Continue,
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
                    //     (window.height() / 5.0) * 3.0,
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
                        PauseMenuButton::RageQuit,
                    ));
                });
        })
        // Rage Controls button
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
                    //     (window.height() / 5.0) * 2.0,
                    //     0.0,
                    // ),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Controls",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        PauseMenuButton::Contols,
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
            if keyboard_input.just_released(KeyCode::ArrowDown)
                || keyboard_input.just_released(KeyCode::Tab)
            {
                debug!("changing button selection to next button with keeb");
                button_selection.0 = Some(PauseMenuButton::next(button));
            } else if keyboard_input.just_released(KeyCode::ArrowUp) {
                debug!("changing button selection to previous button with keeb");
                button_selection.0 = Some(PauseMenuButton::prev(button));
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
                button_selection.0 = Some(PauseMenuButton::Continue);
                button_selection.1 = Interaction::Hovered;
            }
        }
    }
}

/// handles using the mouse to select a button
fn mouse_select(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&PauseMenuButton, With<Text>>,
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
    mut text_query: Query<(&mut Text, &PauseMenuButton)>,
    mut next_state: ResMut<NextState<Screen>>,
    mut button_selection: ResMut<SelectedButton>,
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
                    button_selection.1 = Interaction::Hovered;
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
fn press_button(button: PauseMenuButton) -> Screen {
    match button {
        PauseMenuButton::Continue => Screen::Game,
        PauseMenuButton::Contols => {
            error!("controls menu has yet to programmed");
            Screen::PauseMenu
        }
        PauseMenuButton::RageQuit => Screen::ExitGame,
    }
}

/// handles pausing the game
fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        info!("pausing the game");
        next_state.set(Screen::PauseMenu);
    }
}

/// handles unpausing the game
fn unpause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        info!("resuming the game");
        next_state.set(Screen::Game);
    }
}
