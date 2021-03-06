use wasm_bindgen::prelude::*;

use bevy::{prelude::*, window::WindowMode};
use heron::prelude::PhysicsPlugin;

#[allow(unused)]
mod assets;
mod gameplay;
mod input;
mod utils;

pub const PHYS_SCALE: f32 = 32.0;

#[cfg(not(target_family = "wasm"))]
const WIN_SCALE: f32 = 1.;

#[cfg(target_family = "wasm")]
const WIN_SCALE: f32 = 0.5;

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

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn getWindowWidth() -> f32;
}

#[wasm_bindgen]
pub fn run() {
    let scale = WIN_SCALE;
    // #[cfg(target = "wasm32-unknown-unknown")]
    // {
    //     scale = (getWindowWidth() / 1280f32).clamp(0., 1.);
    // }
    App::new()
        //TODO: Read/write window config to disk
        .insert_resource(WindowDescriptor {
            title: "Toadally Tacos".to_string(),
            width: 1280.0 * scale,
            height: 720.0 * scale,
            vsync: true,
            // scale_factor_override: Some(WIN_SCALE),
            // mode: WindowMode::SizedFullscreen,
            // cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<LoadTime>()
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(start_game))
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
    debug!("Loaded");
    // info!("Took {}s", load_time.0);
}

fn _loading(mut load_time: ResMut<LoadTime>, time: Res<Time>) {
    //do some kind of read here to see how many things need to load
    load_time.0 += time.delta_seconds();
    info!("Loading...");
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let mut ortho_cam = OrthographicCameraBundle::new_2d();
    ortho_cam.transform.scale = Vec3::splat(1. / WIN_SCALE);
    //might need a UI camera here too
    commands.spawn_bundle(ortho_cam).insert(MainCamera);
    // windows
    //     .get_primary_mut()
    //     .unwrap()
    //     .set_cursor_lock_mode(true);
}

fn start_game(mut game_state: ResMut<State<gameplay::GameState>>) {
    game_state
        .set(gameplay::GameState::Catch)
        .expect("Unable to set gamestate");
}
