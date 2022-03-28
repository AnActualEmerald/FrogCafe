use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
struct Backgrounds(HashMap<String, Handle<Image>>);

//--Sprites--//

pub struct FlySprite(pub Handle<Image>);
pub struct CatcherSprite(pub Handle<Image>);

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Backgrounds>()
            .add_startup_system(load_sprites)
            .add_startup_system(load_backgrounds)
            .add_startup_system(load_sounds);
    }
}

fn load_sprites(mut commands: Commands, asset_loader: ResMut<AssetServer>) {
    let fly_handle = FlySprite(asset_loader.load("sprites/FlyPlaceholder.png"));
    let catcher_handle = CatcherSprite(asset_loader.load("sprites/GrabberPlaceholder.png"));

    commands.insert_resource(catcher_handle);
    commands.insert_resource(fly_handle);
}

fn load_sounds() {}

fn load_backgrounds() {}
