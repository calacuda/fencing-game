use std::ops::Not;

use crate::{despawn_buttons, state::Screen, ButtonMarker};
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum ModeSelMenuButton {
    Comp,
    CompAi,
    Lan,
    Spectate,
}

impl ModeSelMenuButton {
    fn next(index: ModeSelMenuButton) -> Self {
        match index {
            Self::Comp => Self::CompAi,
            Self::CompAi => Self::Lan,
            Self::Lan => Self::Spectate,
            Self::Spectate => Self::Comp,
        }
    }

    fn prev(index: ModeSelMenuButton) -> Self {
        match index {
            Self::Comp => Self::Spectate,
            Self::CompAi => Self::Comp,
            Self::Lan => Self::CompAi,
            Self::Spectate => Self::Lan,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
enum ComputerAi {
    Offensive,
    QLearning,
}

impl Not for ComputerAi {
    type Output = ComputerAi;

    fn not(self) -> Self::Output {
        match self {
            Self::Offensive => Self::QLearning,
            Self::QLearning => Self::Offensive,
        }
    }
}

#[derive(Resource)]
struct SelectedButton(Option<ModeSelMenuButton>, Interaction, ComputerAi);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// used to show which player scored the touch
pub struct ModeScreenPlugin;

impl Plugin for ModeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::ModeSelect), spawn_buttons)
            .add_systems(OnExit(Screen::ModeSelect), despawn_buttons)
            .add_systems(Update, keyboard_select.run_if(in_state(Screen::ModeSelect)))
            .add_systems(Update, mouse_select.run_if(in_state(Screen::ModeSelect)))
            .add_systems(
                Update,
                button_selection.run_if(in_state(Screen::ModeSelect)),
            );
    }
}

/// spawns the buttons (and button text) of the score menu
fn spawn_buttons(mut commands: Commands) {
    info!("spawning score touch menu buttons");

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
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Vs. Computer",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ModeSelMenuButton::Comp,
                    ));
                });
        })
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
                            format!("{:?}", ComputerAi::Offensive),
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ModeSelMenuButton::CompAi,
                    ));
                });
        })
        // Controls menu button
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
                            "Vs. Human (LAN)",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ModeSelMenuButton::Lan,
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
                            "Spectate",
                            TextStyle {
                                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ModeSelMenuButton::Spectate,
                    ));
                });
        });

    commands.insert_resource(SelectedButton(
        None,
        Interaction::None,
        ComputerAi::Offensive,
    ));
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
                button_selection.0 = Some(ModeSelMenuButton::next(button));
            } else if keyboard_input.just_released(KeyCode::ArrowUp) {
                debug!("changing button selection to previous button with keeb");
                button_selection.0 = Some(ModeSelMenuButton::prev(button));
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
                button_selection.0 = Some(ModeSelMenuButton::Comp);
                button_selection.1 = Interaction::Hovered;
            }
        }
    }
}

/// handles using the mouse to select a button
fn mouse_select(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&ModeSelMenuButton, With<Text>>,
    mut button_selection: ResMut<SelectedButton>,
) {
    for (interaction, children) in &mut interaction_query {
        let Ok(button_type) = text_query.get_mut(children[0]) else {
            continue;
        };

        match *interaction {
            Interaction::Pressed | Interaction::Hovered => {
                debug!("selecting with mouse");
                *button_selection =
                    SelectedButton(Some(button_type.clone()), *interaction, button_selection.2);
            }
            Interaction::None => {
                debug!("unselecting with mouse");
                *button_selection = SelectedButton(None, *interaction, button_selection.2);
            }
        }
    }
}

/// handles changing the selected buttons collor and text
fn button_selection(
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor, &Children), With<Button>>,
    mut text_query: Query<(&mut Text, &ModeSelMenuButton)>,
    mut next_state: ResMut<NextState<Screen>>,
    mut button_selection: ResMut<SelectedButton>,
) {
    let select_prefix = "> ";

    for (mut color, mut border_color, children) in &mut button_query {
        let Ok((mut text, button_type)) = text_query.get_mut(children[0]) else {
            continue;
        };

        if Some(*button_type) == button_selection.0 {
            match button_selection.1 {
                Interaction::Pressed => {
                    text.sections[0].value = text.sections[0]
                        .value
                        .strip_prefix(select_prefix)
                        .unwrap_or(text.sections[0].value.as_str())
                        .to_string();
                    *color = PRESSED_BUTTON.into();
                    if *button_type == ModeSelMenuButton::CompAi {
                        text.sections[0].value = format!("{:?}", !button_selection.2);
                        button_selection.2 = !button_selection.2;
                    } else {
                        next_state.set(press_button(button_type.clone()));
                    }
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
fn press_button(button: ModeSelMenuButton) -> Screen {
    match button {
        ModeSelMenuButton::Comp => Screen::NewBout,
        // ModeSelMenuButton::Controls => {
        //     error!("controls menu has yet to programmed");
        //     Screen::Welcome
        // }
        // ModeSelMenuButton::HowToPlay => {
        //     error!("how to play menu has yet to programmed");
        //     Screen::Welcome
        // }
        // ModeSelMenuButton::Exit => Screen::ExitGame,
        _ => {
            error!("entry has yet to be programmed");
            Screen::ModeSelect
        }
    }
}
