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
    /// used to announce that a player won the match
    Victory,
}

#[derive(Resource, Debug)]
pub struct GameState {
    pub row: Option<Player>,
    pub p1_score: Score,
    pub p2_score: Score,
    pub lunger: Option<Player>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            row: None,
            p1_score: Score::default(),
            p2_score: Score::default(),
            lunger: None,
        }
    }

    pub fn reset(&mut self) {
        self.row = None;
        self.lunger = None;
    }

    pub fn lunge(&mut self, player: Player) {
        if self.lunger.is_none() {
            self.lunger = Some(player);
        }
    }

    pub fn score_touch(&mut self, player: Player) {
        match player {
            Player::One => {
                if self.p1_score.touches == 14 {
                    info!("match over!");
                    self.p2_score.touches = 0;
                }

                self.p1_score.score_touch();
            }
            Player::Two => {
                if self.p2_score.touches == 14 {
                    info!("match over!");
                    self.p1_score.touches = 0;
                }

                self.p2_score.score_touch();
            }
        }

        // info!(
        //     "player 1: ({}/15) | player 2: ({}/15)",
        //     self.p1_score.touches, self.p2_score.touches
        // );
    }
}

#[derive(Debug, Default, Clone, Copy)]
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
            self.matches += 1;
            self.touches = 0;
        }
    }
}
