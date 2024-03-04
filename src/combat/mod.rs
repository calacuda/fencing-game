use crate::{
    ai::Player2Marker,
    distance,
    fighter::*,
    player::PlayerMarker,
    state::{GameState, Screen},
};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct GaurdIconMarker;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct ScoreBoard;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_scored.run_if(in_state(Screen::Game)))
            .add_systems(Update, side_flip_detect.run_if(in_state(Screen::Game)))
            .add_systems(Update, bounds_limiter.run_if(in_state(Screen::Game)))
            .add_systems(OnExit(Screen::NewBout), score_board)
            // .add_systems(Update, victory_check.run_if(in_state(Screen::Game)))
            .add_systems(Update, position_fighters.run_if(in_state(Screen::Game)));
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
            info!("Player 1 scored");
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
            info!("Player 2 scored");
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
    mut player1_query: Query<
        (&Fighter, &mut Transform),
        (
            With<PlayerMarker>,
            Without<Player2Marker>,
            Without<GaurdIconMarker>,
        ),
    >,
    mut player2_query: Query<
        (&Fighter, &mut Transform),
        (
            With<Player2Marker>,
            Without<PlayerMarker>,
            Without<GaurdIconMarker>,
        ),
    >,
    mut p1_gaurd_query: Query<
        (&mut Transform, &mut TextureAtlas),
        (
            With<GaurdIconMarker>,
            // Without<Fighter>,
            With<PlayerMarker>,
            // Without<Player2Marker>,
        ),
    >,
    mut p2_gaurd_query: Query<
        (&mut Transform, &mut TextureAtlas),
        (
            With<GaurdIconMarker>,
            // Without<Fighter>,
            Without<PlayerMarker>,
            // With<Player2Marker>,
        ),
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let (
        Ok((p1, mut p1_sprite)),
        Ok((p2, mut p2_sprite)),
        Ok((mut p1_gaurd_sprite, mut p1_atlas)),
        Ok((mut p2_gaurd_sprite, mut p2_atlas)),
    ) = (
        player1_query.get_single_mut(),
        player2_query.get_single_mut(),
        p1_gaurd_query.get_single_mut(),
        p2_gaurd_query.get_single_mut(),
    ) {
        let window = window_query.get_single().unwrap();
        // let piste_len = 14.0;
        let (p1_pos, p2_pos) = (p1.position, p2.position);

        p1_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p1_pos),
            window.height() / 2.0,
            0.0,
        );

        p1_gaurd_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p1_pos),
            window.height() / 2.0 + (32.0 * 1.5),
            0.0,
        );

        p1_atlas.index = p1.gaurd.into();

        p2_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p2_pos),
            window.height() / 2.0,
            0.0,
        );

        p2_gaurd_sprite.translation = Vec3::new(
            (window.width() / 2.0) + (32.0 * p2_pos),
            window.height() / 2.0 + (32.0 * 1.5),
            0.0,
        );

        p2_atlas.index = p2.gaurd.into();
    } else {
        // error!(
        //     "could not get one of: player1/2, player1/2 sprite, player1/2 gaurd icon sprite/atlas"
        // )
    }
}

fn score_board(mut commands: Commands, world_state: Res<GameState>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!(
                "({}/15/{})",
                world_state.p1_score.touches, world_state.p1_score.matches
            ),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Vw(12.5),
            left: Val::Vw(12.5),
            ..default()
        }),
        PlayerScore,
        ScoreBoard,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!(
                "({}/15/{})",
                world_state.p2_score.touches, world_state.p2_score.matches
            ),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Vw(12.5), // Val::Px(5.0),
            right: Val::Vw(12.5),
            ..default()
        }),
        ScoreBoard,
    ));
}
