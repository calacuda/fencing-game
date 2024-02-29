use crate::PLAYER_SPEED;
use bevy::prelude::*;

#[derive(Component)]
pub struct Fighter {
    /// what gaurd is this player in.
    pub gaurd: Gaurd,
    /// represents the possistion (in meters) of the front foot, from the center of the piste
    /// (strip). negative values indicate that the fighter is on player ones's side, positive
    /// that they are on player two's side.
    pub position: f32,
    /// represents the players current stance.
    pub stance: Stance,
    /// left or right handed?
    pub handed: Handed,
    /// is this player 1, 2.
    pub player: Player,
    /// who/what controls this figter.
    pub contoller: Controller,
    /// how many touches this fighter has scored.
    pub touches: u8,
    /// how many matches this player has won.
    pub matches: u16,
    /// is this fighter mounting a parry.
    pub parrying: bool,
    //     /// does this fighter have the right of way.
    //     pub right_of_way: bool,
    /// the move that the player is doing. will block other movements untill done.
    pub action: Action,
    /// wether the player is crouched or not.
    pub crouched: bool,
}

impl Fighter {
    /// takes the time struct from the game, returns an amount to move the players sprite.
    pub fn update_movement(&mut self, time: Res<Time>) -> Vec3 {
        let b1 = self.action.blocked();
        let res = self.action.step(time.delta_seconds());
        let b2 = self.action.blocked();

        if b1 && !b2 {
            self.set_action(Move::EnGarde);
        }

        res
    }

    pub fn set_action(&mut self, act: Move) {
        if !self.action.blocked() {
            // info!("setting action -> {:?}", act);
            self.action = Action::from(act);
        }
    }
}

#[derive(Debug)]
pub enum Controller {
    Player,
    Computer,
}

#[derive(Debug)]
pub enum Gaurd {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
pub enum Stance {
    /// swordhand in fort.
    Offence,
    /// swordhand in back.
    Defence,
    /// lunged forward from an offensive stance.
    Lunged,
}

#[derive(Debug)]
pub enum Handed {
    /// fighter is a righty.
    Right,
    /// fighter is a lefty.
    Left,
}

#[derive(Debug)]
pub enum Player {
    /// player one.
    One,
    /// player two.
    Two,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Advance,
    Retreat,
    Jump,
    Lunge,
    EnGarde,
}

impl Into<((f32, f32), Vec3)> for Move {
    fn into(self) -> ((f32, f32), Vec3) {
        match self {
            Self::Advance => ((0.5, 0.35), Vec3::new(0.75, 0.0, 0.0)),
            Self::Retreat => ((0.5, 0.35), Vec3::new(-0.75, 0.0, 0.0)),
            Self::Jump => ((0.75, 0.5), Vec3::new(0.0, 1.25, 0.0)),
            Self::Lunge => ((1.0, 0.5), Vec3::new(0.75, 0.0, 0.0)),
            // f32::NEG_INFINITY makes this non-blocking
            Self::EnGarde => ((f32::NEG_INFINITY, 0.0), Vec3::ZERO),
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub act: Move,
    /// remaining seconds to block for.
    pub block_for: f32,
    /// ammount of movement, measured in seconds
    pub moved: f32,
    pub dir_vec: Vec3,
}

impl From<Move> for Action {
    fn from(value: Move) -> Self {
        let ((block_time, move_time), dir_vec) = value.clone().into();

        Self {
            act: value,
            block_for: block_time,
            moved: move_time,
            dir_vec,
        }
    }
}

impl Action {
    pub fn blocked(&self) -> bool {
        self.block_for >= 0.0
    }

    fn step(&mut self, time_d: f32) -> Vec3 {
        self.block_for -= time_d;
        let res = if self.moved >= 0.0 {
            self.dir_vec * PLAYER_SPEED * time_d
        } else {
            Vec3::ZERO
        };

        self.moved -= time_d;

        res
    }
}
