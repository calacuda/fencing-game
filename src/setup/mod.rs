use crate::{
    combat::{GaurdIconMarker, ScoreBoard},
    state::Screen,
    Fighter,
};
use bevy::{core::FrameCount, prelude::*, window::PrimaryWindow};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(OnEnter(Screen::Game), setup_camera)
            .add_systems(OnEnter(Screen::NewBout), cleanup_after_bout)
            .add_systems(OnEnter(Screen::NewBout), start_game)
            .add_systems(Update, make_visible.run_if(in_state(Screen::Setup)));
    }
}

#[derive(Component)]
struct GameView;

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<Entity, With<GameView>>,
) {
    use bevy::render::camera::ScalingMode;

    camera_query
        .iter()
        .for_each(|cam| commands.entity(cam).despawn());

    let window = window_query.get_single().unwrap();

    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(16.0 * 32.0);

    commands.spawn((camera, GameView));
}

fn make_visible(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if frames.0 == 5 {
        debug!("making window visible");
        window.single_mut().visible = true;
        next_state.set(Screen::Welcome)
    }
}

pub fn cleanup_after_bout(
    mut commands: Commands,
    // The "With<Fighter/GaurdIconMarker/ScoreBoard>" is nessesary because of the generic nature
    // of the "Query<Entity>" part. Without the restiction the camera & EVERYTHING ELSE gets
    // despawned along with the fighters, gaurd icons, and score board. which woudld breaks the
    // game.
    fighters: Query<Entity, With<Fighter>>,
    gaurd_icons: Query<Entity, With<GaurdIconMarker>>,
    score_boards: Query<Entity, With<ScoreBoard>>,
) {
    fighters
        .iter()
        .for_each(|fighter| commands.entity(fighter).despawn());
    gaurd_icons
        .iter()
        .for_each(|icon| commands.entity(icon).despawn());
    score_boards
        .iter()
        .for_each(|board| commands.entity(board).despawn());
}

fn start_game(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Game)
}
