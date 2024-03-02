use crate::{
    distance,
    fighter::*,
    player::PlayerMarker,
    state::{GameState, Screen},
};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_scored.run_if(in_state(Screen::Game)));
        // .add_systems(Update, parried.run_if(in_state(Screen::Game)));
    }
}

fn touch_scored(
    player1_query: Query<&Fighter, With<PlayerMarker>>,
    player2_query: Query<&Fighter, Without<PlayerMarker>>,
    // time: Res<Time>,
    mut world_state: ResMut<GameState>,
    // window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if let (Ok(p1), Ok(p2)) = (player1_query.get_single(), player2_query.get_single())
        && (p1.lunged() || p2.lunged())
    {
        let range = 2.55;
        let distance = distance(p1.position, p2.position);

        debug!("{}, {}, distance => {}", p1.position, p2.position, distance);
        // if !p1.lunged() && !p2.lunged() {}
        if p1.lunged()
            && world_state.lunger == Some(Player::One)
            && world_state.row == Some(Player::One)
            // && lunge_time.elapsed_seconds() >= 0.5
            && distance <= range
        {
            info!("Player 1 scored the touch");
            world_state.p1_score.score_touch();
            world_state.reset();
            next_state.set(Screen::NewBout);
        }
        if p2.lunged()
            // && world_state.lunger == Some(Player::Two)
            && world_state.row == Some(Player::Two)
            // && lunge_time.elapsed_seconds() >= 0.5
            && distance <= range
        {
            info!("Player 2 scored the touch");
            world_state.p2_score.score_touch();
            world_state.reset();
            next_state.set(Screen::NewBout);
        }
    }
}

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
