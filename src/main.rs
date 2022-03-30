use bevy::prelude::*;
use bevy::window::WindowMode;
use heron::prelude::PhysicsPlugin;

#[allow(unused)]
mod assets;
mod gameplay;
mod input;

pub const PHYS_SCALE: f32 = 32.0;

const WIN_SCALE: f32 = 1.;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    Pause,
    Game,
}

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        //TODO: Read/write window config to disk
        .insert_resource(WindowDescriptor {
            title: "Toadally Tacos".to_string(),
            width: 1280.0 * WIN_SCALE,
            height: 720.0 * WIN_SCALE,
            // mode: WindowMode::SizedFullscreen,
            // cursor_locked: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(start_game))
        .add_plugin(assets::AssetPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut ortho_cam = OrthographicCameraBundle::new_2d();
    ortho_cam.transform.scale = Vec3::splat(1. / WIN_SCALE);
    //might need a UI camera here too
    commands.spawn_bundle(ortho_cam).insert(MainCamera);
}

fn start_game(mut game_state: ResMut<State<gameplay::GameState>>) {
    game_state
        .set(gameplay::GameState::Catch)
        .expect("Unable to set gamestate");
}
