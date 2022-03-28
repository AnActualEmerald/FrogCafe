use bevy::{ecs::system::ResMutState, prelude::*};
// use bevy::window::WindowMode;

#[allow(unused)]
mod assets;
mod gameplay;
mod input;

pub const PHYS_SCALE: f32 = 32.0;

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
            // width: 1280.0,
            // height: 720.0,
            cursor_locked: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(start_game))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(done_loading))
        .add_plugin(assets::AssetPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(gameplay::GameplayPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    //might need a UI camera here too
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn done_loading(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::Game).unwrap() // this should go to the main menu when that is implemented
}

fn start_game(mut game_state: ResMut<State<gameplay::GameState>>) {
    game_state
        .set(gameplay::GameState::Catch)
        .expect("Unable to set gamestate");
}
