use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
struct Backgrounds(HashMap<String, Handle<Image>>);

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Backgrounds>()
            .add_startup_system(load_sprites)
            .add_startup_system(load_backgrounds)
            .add_startup_system(load_sounds);
    }
}

fn load_sprites() {}

fn load_sounds() {}

fn load_backgrounds() {}
