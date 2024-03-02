use crate::Player;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Screen {
    #[default]
    Setup,
    Welcome,
    ModeSelect,
    Game,
    PauseMenu,
    NewBout,
    /// used to announce that a player scored the touch
    ScoreBoard,
}

#[derive(Resource, Debug)]
pub struct GameState {
    // pub screen: Screen,
    pub row: Option<Player>,
    pub p1_score: Score,
    pub p2_score: Score,
    pub lunger: Option<Player>,
    // pub lunge_time: Option<Time>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            // screen: Screen::Welcome,
            row: None,
            p1_score: Score::default(),
            p2_score: Score::default(),
            lunger: None,
            // lunge_time: None,
        }
    }

    pub fn reset(&mut self) {
        self.row = None;
        self.lunger = None;
        // self.lunge_time = None;
    }

    pub fn lunge(&mut self, player: Player) {
        if self.lunger.is_none() {
            self.lunger = Some(player);
        }
    }
}

#[derive(Debug, Default)]
pub struct Score {
    /// how many touches for the current match this fighter has scored.
    pub touches: u8,
    /// how many matches this player has won.
    pub matches: u16,
}

impl Score {
    pub fn score_touch(&mut self) {
        self.touches += 1;

        if self.touches == 15 {
            info!("match over.");
            self.matches += 1;
            self.touches = 0;
        }
    }
}
