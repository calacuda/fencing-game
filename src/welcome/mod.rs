use crate::{
    despawn_buttons,
    setup::cleanup_after_bout,
    state::{GameState, Screen},
    ButtonMarker,
};
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum WelcomeMenuButton {
    Play,
    Controls,
    HowToPlay,
    Exit,
}

impl WelcomeMenuButton {
    fn next(index: WelcomeMenuButton) -> Self {
        match index {
            Self::Play => Self::Controls,
            Self::Controls => Self::HowToPlay,
            Self::HowToPlay => Self::Exit,
            Self::Exit => Self::Play,
        }
    }

    fn prev(index: WelcomeMenuButton) -> Self {
        match index {
            Self::Play => Self::Exit,
            Self::Controls => Self::Play,
            Self::HowToPlay => Self::Controls,
            Self::Exit => Self::HowToPlay,
        }
    }
}

#[derive(Resource)]
struct SelectedButton(Option<WelcomeMenuButton>, Interaction);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// used to show which player scored the touch
pub struct WelcomeScreenPlugin;

impl Plugin for WelcomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Welcome), spawn_buttons)
            .add_systems(OnEnter(Screen::Welcome), cleanup_after_bout)
            .add_systems(OnEnter(Screen::Welcome), cleanup_world_state)
            .add_systems(OnExit(Screen::Welcome), despawn_buttons)
            .add_systems(Update, keyboard_select.run_if(in_state(Screen::Welcome)))
            .add_systems(Update, mouse_select.run_if(in_state(Screen::Welcome)))
            .add_systems(Update, button_selection.run_if(in_state(Screen::Welcome)));
    }
}

/// spawns the buttons (and button text) of the score menu
fn spawn_buttons(mut commands: Commands) {
    info!("spawning Welcome menu buttons");

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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Pixle Fencer",
                TextStyle {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        })
        // spawn play Button
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
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Play",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        WelcomeMenuButton::Play,
                    ));
                });
        })
        // Controls button
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
                        WelcomeMenuButton::Controls,
                    ));
                });
        })
        // How to play menu
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
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "How To Play",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        WelcomeMenuButton::HowToPlay,
                    ));
                });
        })
        // exit button
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
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        WelcomeMenuButton::Exit,
                    ));
                });
        });

    commands.insert_resource(SelectedButton(None, Interaction::None));
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
                button_selection.0 = Some(WelcomeMenuButton::next(button));
            } else if keyboard_input.just_released(KeyCode::ArrowUp) {
                debug!("changing button selection to previous button with keeb");
                button_selection.0 = Some(WelcomeMenuButton::prev(button));
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
                button_selection.0 = Some(WelcomeMenuButton::Play);
                button_selection.1 = Interaction::Hovered;
            }
        }
    }
}

/// handles using the mouse to select a button
fn mouse_select(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&WelcomeMenuButton, With<Text>>,
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
    mut text_query: Query<(&mut Text, &WelcomeMenuButton)>,
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
fn press_button(button: WelcomeMenuButton) -> Screen {
    match button {
        WelcomeMenuButton::Play => Screen::ModeSelect,
        WelcomeMenuButton::Controls => {
            error!("controls menu has yet to programmed");
            Screen::Welcome
        }
        WelcomeMenuButton::HowToPlay => {
            error!("how to play menu has yet to programmed");
            Screen::Welcome
        }
        WelcomeMenuButton::Exit => Screen::ExitGame,
    }
}

fn cleanup_world_state(mut world_state: ResMut<GameState>) {
    world_state.reset();
    world_state.reset_scores();
}
