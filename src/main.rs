use bevy::prelude::{App, ClearColor, Color, DefaultPlugins, WindowDescriptor};
// use bevy::window::WindowMode;

#[allow(unused)]
mod assets;
mod gameplay;

pub const PHYS_SCALE: f32 = 32.0;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    Pause,
    Game,
}

fn main() {
    App::new()
        //TODO: Read/write window config to disk
        .insert_resource(WindowDescriptor {
            title: "Toadally Tacos".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_state(AppState::MainMenu)
        .add_plugin(assets::AssetPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
