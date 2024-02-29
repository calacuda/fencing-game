use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Screen {
    #[default]
    Setup,
    Welcome,
    ModeSelect,
    Game,
    PauseMenu,
}

#[derive(Resource, Debug)]
pub struct GameState {
    // pub screen: Screen,
    pub row: Option<crate::fighter::Player>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            // screen: Screen::Welcome,
            row: None,
        }
    }
}
