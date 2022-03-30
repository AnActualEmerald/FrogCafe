use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use std::collections::HashMap;

#[derive(AssetCollection)]
pub struct Sprites {
    #[asset(path = "sprites/FlyPlaceholder.png")]
    pub fly: Handle<Image>,
    #[asset(path = "sprites/GrabberPlaceholder.png")]
    pub grabber: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct Backgrounds {}

#[derive(AssetCollection)]
pub struct Sounds {}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(AppState::Loading)
            .continue_to_state(AppState::Game)
            .with_collection::<Sprites>()
            .with_collection::<Backgrounds>()
            .with_collection::<Sounds>()
            .build(app);
    }
}
