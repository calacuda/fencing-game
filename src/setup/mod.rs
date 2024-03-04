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
            .add_systems(Update, start_bout.run_if(in_state(Screen::NewBout)))
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

    // window.scale_factor()
    let mut camera = Camera2dBundle::default();
    // For this example, let's make the screen/window height correspond to
    // 1600.0 world units. The width will depend on the aspect ratio.
    // camera.projection.scaling_mode = ScalingMode::FixedHorizontal(800.0);
    camera.transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    // camera.projection.scaling_mode = ScalingMode::FixedVertical(200.0);
    // camera.projection.scaling_mode = ScalingMode::FixedVertical(300.0);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(16.0 * 32.0);

    commands.spawn((camera, GameView));
}

fn make_visible(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    // The delay may be different for your app or system.
    if frames.0 == 5 {
        // info!("making window visible");
        window.single_mut().visible = true;
        // TODO: change this once welcome and game mode selection screens get written.
        next_state.set(Screen::NewBout)
    }
}

fn start_bout(
    mut commands: Commands,
    mut next_state: ResMut<NextState<Screen>>,
    fighters: Query<Entity, With<Fighter>>,
    gaurd_icons: Query<Entity, With<GaurdIconMarker>>,
    score_boards: Query<Entity, With<ScoreBoard>>,
    // spawned_fighters: Query<Entity>,
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

    next_state.set(Screen::Game)
}
