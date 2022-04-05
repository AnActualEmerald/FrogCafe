use bevy::prelude::*;
use bevy::window::WindowMode;
use heron::prelude::PhysicsPlugin;

#[allow(unused)]
mod assets;
mod gameplay;
mod input;

pub const PHYS_SCALE: f32 = 32.0;

// const WIN_SCALE: f64 = 1.;

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
            width: 1280.0,
            height: 720.0,
            // scale_factor_override: Some(WIN_SCALE),
            // mode: WindowMode::SizedFullscreen,
            // cursor_locked: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<LoadTime>()
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(start_game))
        // .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading))
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(loaded))
        .add_plugin(assets::AssetPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default)]
struct LoadTime(f32);

fn loaded(_load_time: Res<LoadTime>) {
    info!("Loaded");
    // info!("Took {}s", load_time.0);
}

fn _loading(mut load_time: ResMut<LoadTime>, time: Res<Time>) {
    //do some kind of read here to see how many things need to load
    load_time.0 += time.delta_seconds();
    info!("Loading...");
}

fn setup(mut commands: Commands) {
    let mut ortho_cam = OrthographicCameraBundle::new_2d();
    ortho_cam.transform.scale = Vec3::splat(1.);
    //might need a UI camera here too
    commands.spawn_bundle(ortho_cam).insert(MainCamera);
}

fn start_game(mut game_state: ResMut<State<gameplay::GameState>>) {
    game_state
        .set(gameplay::GameState::Catch)
        .expect("Unable to set gamestate");
}
