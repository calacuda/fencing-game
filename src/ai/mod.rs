use crate::{
    combat::GaurdIconMarker,
    distance,
    fighter::*,
    player::{P2Timer, PlayerMarker},
    state::{GameState, Screen},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player2Marker;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(Screen::NewBout), spawn_fighter_two)
            .add_systems(OnExit(Screen::NewBout), setup_p2_timer)
            .add_systems(OnExit(Screen::NewBout), spawn_p2_gaurd_icon)
            .add_systems(Update, fighter_2_movement.run_if(in_state(Screen::Game)))
            .add_systems(Update, reset_fighter.run_if(in_state(Screen::Game)));
    }
}

fn spawn_fighter_two(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/fighter-sprites.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Fighter {
            gaurd: Gaurd::Left,
            position: 2.0,
            stance: Stance::Offence,
            handed: Handed::Right,
            player: Player::Two,
            // contoller: Controller::Computer,
            // touches: 0,
            // matches: 0,
            parrying: false,
            action: Action::from(Move::EnGarde),
            crouched: false,
        },
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            sprite: Sprite {
                // custom_size: Some(Vec2::new(100., 100.)),
                flip_x: true,
                ..default()
            },
            ..default()
        },
        Player2Marker,
    ));
}

fn spawn_p2_gaurd_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/gaurd-icons.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // info!("spawning p2 gaurd icon");
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 32.)),
                flip_x: true,
                flip_y: true,
                ..default()
            },
            ..default()
        },
        GaurdIconMarker,
    ));
}

fn setup_p2_timer(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(P2Timer(time.clone()));
}

pub fn fighter_2_movement(
    mut fighter_2_query: Query<(&mut Fighter, &mut TextureAtlas), Without<PlayerMarker>>,
    player1_query: Query<&Fighter, With<PlayerMarker>>,
    mut p2_timer: ResMut<P2Timer>,
    time: Res<Time>,
    mut world_state: ResMut<GameState>, // needed to change who has the Right of way
) {
    if let (Ok((mut fighter, mut atlas)), Ok(player1)) =
        (fighter_2_query.get_single_mut(), player1_query.get_single())
    {
        if !fighter.action.blocked() && p2_timer.0.elapsed_seconds() >= 0.25 {
            // do computer player action
            let distance = distance(player1.position, fighter.position);
            let prev_gaurd = fighter.gaurd.clone();

            if distance >= 4.0 {
                // advance if player too far,
                fighter.set_action(Move::Advance);
                world_state.row = Some(Player::Two);
            } else if distance <= 3.25 && world_state.row == Some(Player::One) {
                // retreat if player too close,
                fighter.set_action(Move::Retreat);

                if world_state.row == Some(Player::Two) {
                    world_state.row = None;
                }
            } else if distance <= 1.75 && world_state.row == Some(Player::Two) {
                // lunge if in range
                fighter.set_action(Move::Lunge);
                // world_state.row = None;
                atlas.index += 1;
                world_state.lunger = Some(Player::Two);
            } else if player1.lunged() && world_state.row == Some(Player::One) && distance <= 1.75 {
                // parry if enemy lunges and has right of way
                // fighter.gaurd = match fighter.gaurd {
                //     Gaurd::Up => Gaurd::Left,
                //     Gaurd::Down => Gaurd::Right,
                //     Gaurd::Left => Gaurd::Down,
                //     Gaurd::Right => Gaurd::Up,
                // };
                // TODO: make fail (but only sometimes)
                fighter.gaurd = player1.gaurd;

                world_state.row = Some(Player::Two);
            } else if world_state.row == Some(Player::Two)
                || world_state.row.is_none()
                || player1.action.act == Move::EnGarde
            {
                fighter.set_action(Move::Advance);
            }
            // else {
            //     fighter.set_action(Move::EnGarde);
            // }

            if prev_gaurd != fighter.gaurd
                && player1.lunged()
                && fighter.gaurd.parries(player1.gaurd)
            {
                fighter.parrying = true;
            } else if !(player1.lunged() && fighter.gaurd.parries(player1.gaurd)) {
                fighter.parrying = false;
            }
        }

        let pos_d = fighter.update_movement(time.clone());
        // info!("{:?} -> {}", player.action.act, movement);

        fighter.position -= pos_d;

        if p2_timer.0.elapsed_seconds() >= 0.25 {
            *p2_timer = P2Timer(time.clone());
        }
    }
}

fn reset_fighter(mut player_query: Query<(&Fighter, &mut TextureAtlas), Without<PlayerMarker>>) {
    if let Ok((player, mut atlas)) = player_query.get_single_mut()
        && player.action.act != Move::Lunge
        && atlas.index > 0
    {
        // info!("resetting player sprite to standing position");
        atlas.index = 0;
    }
}
