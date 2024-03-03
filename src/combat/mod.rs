use crate::{
    distance,
    fighter::*,
    player::PlayerMarker,
    state::{GameState, Screen},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_scored.run_if(in_state(Screen::Game)))
            .add_systems(Update, side_flip_detect.run_if(in_state(Screen::Game)))
            .add_systems(Update, bounds_limiter.run_if(in_state(Screen::Game)))
            // .add_systems(Update, victory_check.run_if(in_state(Screen::Game)))
            .add_systems(Update, position_fighters.run_if(in_state(Screen::Game)));
        // .add_systems(Update, parried.run_if(in_state(Screen::Game)));
    }
}

fn touch_scored(
    player1_query: Query<&Fighter, With<PlayerMarker>>,
    player2_query: Query<&Fighter, Without<PlayerMarker>>,
    mut world_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if let (Ok(p1), Ok(p2)) = (player1_query.get_single(), player2_query.get_single())
        && (p1.lunged() || p2.lunged())
    {
        let range = 0.5;
        let distance = distance(p1.position, p2.position);

        debug!("{}, {}, distance => {}", p1.position, p2.position, distance);
        // if !p1.lunged() && !p2.lunged() {}
        if p1.lunged()
            && world_state.lunger == Some(Player::One)
            && world_state.row == Some(Player::One)
            // && lunge_time.elapsed_seconds() >= 0.5
            && !p2.parrying
            && distance <= range
        {
            info!("Player 1 scored the touch");
            world_state.score_touch(Player::One);
            world_state.reset();
            next_state.set(Screen::NewBout);
        }
        if p2.lunged()
            && world_state.lunger == Some(Player::Two)
            && world_state.row == Some(Player::Two)
            // && lunge_time.elapsed_seconds() >= 0.5
            && !p1.parrying
            && distance <= range
        {
            info!("Player 2 scored the touch");
            world_state.score_touch(Player::Two);
            world_state.reset();
            next_state.set(Screen::NewBout);
        }
    }
}

fn side_flip_detect(
    player1_query: Query<&Fighter, With<PlayerMarker>>,
    player2_query: Query<&Fighter, Without<PlayerMarker>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if let (Ok(p1), Ok(p2)) = (player1_query.get_single(), player2_query.get_single())
        && p1.position > p2.position
        && !p1.lunged()
        && !p1.lunged()
    {
        info!("players switched moved passed each other, resseting.");
        next_state.set(Screen::NewBout);
    }
}

fn bounds_limiter(
    mut player1_query: Query<&mut Fighter, With<PlayerMarker>>,
    mut player2_query: Query<&mut Fighter, Without<PlayerMarker>>,
) {
    if let (Ok(mut p1), Ok(mut p2)) = (
        player1_query.get_single_mut(),
        player2_query.get_single_mut(),
    ) {
        let (p1_pos, p2_pos) = (p1.position, p2.position);

        if p1_pos < -7.0 {
            info!("player 1 tried to move out of bounds, limiting movement.");
            p1.position = -7.0;
        }

        if p2_pos > 7.0 {
            info!("player 2 tried to move out of bounds, limiting movement.");
            p2.position = 7.0;
        }
    }
}

fn position_fighters(
    mut player1_query: Query<(&Fighter, &mut Transform), With<PlayerMarker>>,
    mut player2_query: Query<(&Fighter, &mut Transform), Without<PlayerMarker>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let (Ok((p1, mut p1_sprite)), Ok((p2, mut p2_sprite))) = (
        player1_query.get_single_mut(),
        player2_query.get_single_mut(),
    ) {
        let window = window_query.get_single().unwrap();
        // let piste_len = 14.0;
        let (p1_pos, p2_pos) = (p1.position, p2.position);

        p1_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p1_pos),
            window.height() / 2.0,
            0.0,
        );

        p2_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p2_pos),
            window.height() / 2.0,
            0.0,
        );
    }
}

// fn victory_check(mut world_state: ResMut<GameState>) {
//     if
// }

// fn reset_players(
//     p1: (&mut Fighter, &mut Transform),
//     p2: (&mut Fighter, &mut Transform),
//     window: &Window,
// ) {
//     // reset p1 & p2 locations
//     p1.0.position = -2.0;
//     p2.0.position = 2.0;
//     // reset p1 & p2 sprite locations
//     *p1.1 = Transform::from_xyz(
//         (window.width() / 2.0) - (32.0 * 1.0),
//         window.height() / 4.0,
//         0.0,
//     );
//     *p2.1 = Transform::from_xyz(
//         (window.width() / 2.0) + (32.0 * 1.0),
//         window.height() / 4.0,
//         0.0,
//     );
//     // reset action
//     p1.0.action = Action::from(Move::EnGarde);
//     p2.0.action = Action::from(Move::EnGarde);
// }

// pub fn rock_paper_scisors(g1: Gaurd, g2: Gaurd) -> bool {
//
// }
