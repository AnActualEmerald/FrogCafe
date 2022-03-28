use crate::assets::{CatcherSprite, FlySprite};
use crate::PHYS_SCALE;
use bevy::prelude::*;

use super::GameState;

pub fn init(mut commands: Commands, catcher_sprite: Res<CatcherSprite>) {}

//put input handling and actual gameplay stuff here
pub fn update_set(state: GameState) -> SystemSet {
    SystemSet::on_update(state)
}

//despawn relevant entities here
pub fn exit_set(state: GameState) -> SystemSet {
    SystemSet::on_exit(state)
}
