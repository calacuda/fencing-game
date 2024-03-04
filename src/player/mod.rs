use crate::{
    combat::GaurdIconMarker,
    fighter::*,
    state::{GameState, Screen},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Resource)]
pub struct P2Timer(pub Time);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(Screen::NewBout), spawn_fighter_one)
            .add_systems(OnExit(Screen::NewBout), spawn_p1_gaurd_icon)
            .add_systems(Update, player_movement.run_if(in_state(Screen::Game)))
            .add_systems(Update, reset_player.run_if(in_state(Screen::Game)))
            .add_systems(Update, player_blade_play.run_if(in_state(Screen::Game)));
    }
}

fn spawn_fighter_one(
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
            position: -2.0,
            stance: Stance::Offence,
            handed: Handed::Right,
            player: Player::One,
            // contoller: Controller::Player,
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
            ..default()
        },
        PlayerMarker,
    ));
}

fn spawn_p1_gaurd_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/gaurd-icons.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(50.0, 50.0), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },

            ..default()
        },
        GaurdIconMarker,
        PlayerMarker,
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Fighter, &mut TextureAtlas), With<PlayerMarker>>,
    time: Res<Time>,
    mut world_state: ResMut<GameState>,
) {
    if let Ok((mut player, mut atlas)) = player_query.get_single_mut() {
        if !player.action.blocked() {
            if keyboard_input.pressed(KeyCode::KeyA) {
                // move left (retreat)
                player.set_action(Move::Retreat);

                if world_state.row == Some(Player::One) {
                    world_state.row = None;
                }
            } else if keyboard_input.pressed(KeyCode::KeyD) {
                // move right (advance)
                player.set_action(Move::Advance);

                if world_state.row.is_none() {
                    world_state.row = Some(Player::One);
                }
            } else if keyboard_input.pressed(KeyCode::KeyW) {
                // lunge
                player.set_action(Move::Lunge);
                atlas.index = 1;
                world_state.lunge(Player::One);
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                // crouch
                error!("Crouching not yet implemented yet");
            }
        }

        let pos_d = player.update_movement(time.clone());
        // info!("{:?} -> {}", player.action.act, movement);

        player.position += pos_d;
    } else {
        error!("no player found");
    }
}

fn reset_player(mut player_query: Query<(&Fighter, &mut TextureAtlas), With<PlayerMarker>>) {
    if let Ok((player, mut atlas)) = player_query.get_single_mut()
        && player.action.act != Move::Lunge
        && atlas.index > 0
    {
        // info!("resetting player sprite to standing position");
        atlas.index = 0;
    }
}

pub fn player_blade_play(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Fighter, With<PlayerMarker>>,
    player2_query: Query<&Fighter, Without<PlayerMarker>>,
    _time: Res<Time>,
    mut world_state: ResMut<GameState>,
) {
    if let (Ok(mut player), Ok(player2)) =
        (player_query.get_single_mut(), player2_query.get_single())
    {
        if !player.action.blocked() {
            let prev_gaurd = player.gaurd.clone();

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                player.gaurd = Gaurd::Left;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                player.gaurd = Gaurd::Right;
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                player.gaurd = Gaurd::Up;
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                player.gaurd = Gaurd::Down;
            }

            if prev_gaurd != player.gaurd {
                debug!("player 1 gaurd change: {:?}", player.gaurd);
            }

            if prev_gaurd != player.gaurd && player2.lunged() && player.gaurd.parries(player2.gaurd)
            {
                player.parrying = true;
            } else if !(player2.lunged() && player.gaurd.parries(player2.gaurd)) {
                player.parrying = false;
            }

            if Some(Player::Two) == world_state.lunger && prev_gaurd != player.gaurd
            // && player.gaurd.parries(player2.gaurd)
            {
                info!("player 1 parried and stole right of way");
                world_state.lunger = None;
                world_state.row = Some(Player::One);
            }
        }
    }
}
