use crate::Player;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Screen {
    /// setup the window and stuff, is a transient state only entered once when the game is
    /// initially launched
    #[default]
    Setup,
    /// the welcome screen (choose between playing the game (ModeSelect), view/edit controls
    /// (Controls), or Rules (HowToPlay))
    Welcome,
    /// choose to duel the computer, duel a human (over a LAN connection), or spectate a human vs.
    /// human match
    ModeSelect,
    /// represents that the bout is being ongoing
    Game,
    /// the pause menu (only accessable when dueling the computer)
    PauseMenu,
    /// used to clean up from last bout and setup for next bout.
    NewBout,
    /// used to announce that a player scored the touch
    ScoreBoard,
    /// used to announce that a player won the match
    Victory,
    /// used to show/edit the controls,
    Controls,
    /// explains the rules/icons/interface to the player
    HowToPlay,
    /// announces which player scored the touch
    TouchScored,
    /// announces which player scored the touch
    MatchWon,
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

    pub fn score_touch(&mut self, player: Player) -> Screen {
        let mut next_state: Screen = Screen::TouchScored;

        match player {
            Player::One => {
                if self.p1_score.touches == 14 {
                    info!("match over!");
                    self.p2_score.touches = 0;
                    next_state = Screen::Victory;
                }

                self.p1_score.score_touch();
            }
            Player::Two => {
                if self.p2_score.touches == 14 {
                    info!("match over!");
                    self.p1_score.touches = 0;
                    next_state = Screen::Victory;
                }

                self.p2_score.score_touch();
            }
        }

        // TODO: return "next_state" once Victory and TouchScored screens are implemented.

        // next_state

        // TODO: remove below once Victory and TouchScored screens are implemented.
        Screen::NewBout
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
