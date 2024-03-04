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
    // /// who/what controls this figter.
    // pub contoller: Controller,
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
    pub fn update_movement(&mut self, time: Time) -> f32 {
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

    pub fn lunged(&self) -> bool {
        self.action.act == Move::Lunge
    }
}

#[derive(Debug)]
pub enum Controller {
    Player,
    Computer,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gaurd {
    Right,
    Down,
    Left,
    Up,
}

impl Gaurd {
    pub fn parries(&self, other: Self) -> bool {
        // match (self, other) {
        //     (Self::Up, Self::Up) => true,
        //     _ => false,
        // }
        *self == other
    }
}

impl Into<usize> for Gaurd {
    fn into(self) -> usize {
        match self {
            Self::Right => 2,
            Self::Left => 1,
            Self::Up => 3,
            Self::Down => 0,
        }
    }
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

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    /// player one.
    One,
    /// player two.
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Advance,
    Retreat,
    // Jump,
    Lunge,
    EnGarde,
    // TODO: add DoubleAdvance and DoubleRetreat
}

impl Into<((f32, f32), Vec3)> for Move {
    fn into(self) -> ((f32, f32), Vec3) {
        match self {
            Self::Advance => ((0.5, 0.35), Vec3::new(0.75, 0.0, 0.0)),
            Self::Retreat => ((0.5, 0.35), Vec3::new(-0.75, 0.0, 0.0)),
            // Self::Jump => ((0.75, 0.5), Vec3::new(0.0, 1.25, 0.0)),
            Self::Lunge => ((0.75, 0.2), Vec3::new(0.75, 0.0, 0.0)),
            // f32::NEG_INFINITY makes this non-blocking
            Self::EnGarde => ((f32::NEG_INFINITY, 0.0), Vec3::ZERO),
        }
    }
}

impl From<Move> for f32 {
    fn from(value: Move) -> Self {
        match value {
            Move::Advance => 1.0,
            Move::Retreat => 1.0,
            // Move::Jump => 1.5,
            Move::Lunge => 4.0,
            Move::EnGarde => 0.0,
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

    fn step(&mut self, time_d: f32) -> f32 {
        self.block_for -= time_d;
        let res = if self.moved >= 0.0 {
            (self.dir_vec * PLAYER_SPEED * f32::from(self.act) * time_d)[0]
        } else {
            0.0
        };

        self.moved -= time_d;

        res
    }
}
