#![feature(let_chains)]
use bevy::{prelude::*, window::PresentMode};
use fighter::*;
use state::{GameState, Screen};

mod ai;
mod combat;
pub mod fighter;
mod player;
mod setup;
mod state;

pub const PLAYER_SPEED: f32 = 0.75;

fn main() {
    App::new()
        .insert_resource(GameState::new())
        .init_state::<Screen>()
        .add_plugins(setup::SetupPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(ai::AiPlugin)
        .add_plugins(combat::CombatPlugin)
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

// fn print_fighters_state(query: Query<&Fighter>) {
//     for player in query.iter() {
//         info!(
//             "player: {:?}, [touches, matches]: ({}, {})",
//             player.player, player.touches, player.matches
//         );
//     }
// }

pub fn distance(pos1: f32, pos2: f32) -> f32 {
    (pos2 - pos1).powf(2.0).sqrt()
    // info!("distance: {res}");
    // res
}
