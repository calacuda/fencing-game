#![feature(let_chains)]
use bevy::{
    asset::LoadedFolder,
    core::FrameCount,
    prelude::*,
    render::texture::ImageSampler,
    window::{PresentMode, PrimaryWindow},
};
use fighter::*;

pub mod fighter;
mod state;

pub const PLAYER_SPEED: f32 = 40.0;

#[derive(Component)]
struct PlayerMarker;

fn main() {
    App::new()
        .insert_resource(state::GameState::new())
        .init_state::<state::Screen>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(state::Screen::Setup), start_game)
        .add_systems(OnEnter(state::Screen::Game), spawn_fighter_one)
        .add_systems(OnEnter(state::Screen::Game), spawn_fighter_two)
        .add_systems(
            Update,
            player_movement.run_if(in_state(state::Screen::Game)),
        )
        .add_systems(Update, reset_player)
        .add_systems(Update, make_visible)
        // .add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // title: "I am a window!".into(),
                        // name: Some("bevy.app".into()),
                        resolution: (500., 281.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run()
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 5 {
        window.single_mut().visible = true;
    }
}

fn start_game(mut next_state: ResMut<NextState<state::Screen>>) {
    next_state.set(state::Screen::Game)
}

fn spawn_fighter_one(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
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
            contoller: Controller::Player,
            touches: 0,
            matches: 0,
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
            transform: Transform::from_xyz(
                (window.width() / 2.0) - 40.0,
                window.height() / 4.0,
                0.0,
            ),
            // sprite: Sprite {
            //     custom_size: Some(Vec2::new(100., 100.)),
            //     ..default()
            // },
            ..default()
        },
        // SpriteBundle {
        //     texture: asset_server.load("sprites/player-1.png"),
        //     transform: Transform::from_xyz(
        //         (window.width() / 2.0) - 40.0,
        //         window.height() / 4.0,
        //         0.0,
        //     ),
        //     sprite: Sprite {
        //         custom_size: Some(Vec2::new(100., 100.)),
        //         ..default()
        //     },
        //     ..default()
        // },
        PlayerMarker,
    ));
}

fn spawn_fighter_two(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    // let texture = asset_server.load("sprites/fighter-sprites.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Fighter {
            gaurd: Gaurd::Left,
            position: 2.0,
            stance: Stance::Offence,
            handed: Handed::Right,
            player: Player::Two,
            contoller: Controller::Computer,
            touches: 0,
            matches: 0,
            parrying: false,
            action: Action::from(Move::EnGarde),
            crouched: false,
        },
        SpriteBundle {
            texture: asset_server.load("sprites/player-2.png"),
            transform: Transform::from_xyz(
                (window.width() / 2.0) + 40.0,
                window.height() / 4.0,
                0.0,
            ),
            // sprite: Sprite {
            //     custom_size: Some(Vec2::new(100., 100.)),
            //     ..default()
            // },
            ..default()
        },
    ));
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

// fn print_fighters_state(query: Query<&Fighter>) {
//     for player in query.iter() {
//         info!(
//             "player: {:?}, [touches, matches]: ({}, {})",
//             player.player, player.touches, player.matches
//         );
//     }
// }

fn setup_camera(mut commands: Commands, window_query: Query<&mut Window, With<PrimaryWindow>>) {
    use bevy::render::camera::ScalingMode;

    let window = window_query.get_single().unwrap();

    // window.scale_factor()
    let mut camera = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    // camera.projection.scaling_mode = ScalingMode::FixedHorizontal(800.0);
    camera.projection.scaling_mode = ScalingMode::FixedVertical(800.0);
    camera.transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);

    commands.spawn(camera);
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Fighter, &mut Transform, &mut TextureAtlas), With<PlayerMarker>>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut transform, mut atlas)) = player_query.get_single_mut() {
        if !player.action.blocked() {
            if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
                // move left (retreat)
                // Vec2::new(-1.0, 0.0)
                player.set_action(Move::Retreat);
            } else if keyboard_input.pressed(KeyCode::KeyD)
                || keyboard_input.pressed(KeyCode::ArrowRight)
            {
                // move right (advance)
                // Vec2::new(1.0, 0.0)
                player.set_action(Move::Advance);
            } else if keyboard_input.pressed(KeyCode::KeyW)
                || keyboard_input.pressed(KeyCode::ArrowUp)
            {
                // lunge
                // Vec3::new(0.0, 1.0, 0.0)
                player.set_action(Move::Lunge);
                atlas.index = 1;
                // Vec3::ZERO
            } else if keyboard_input.pressed(KeyCode::KeyS)
                || keyboard_input.pressed(KeyCode::ArrowDown)
            {
                // crouch
                error!("Crouching not yet implemented yet");
                // Vec3::new(0.0, -1.0, 0.0)
            }
        }

        let movement = player.update_movement(time);
        // info!("{:?} -> {}", player.action.act, movement);

        player.position += movement[0];
        transform.translation += movement;
    } else {
        error!("no player found");
    }
}
