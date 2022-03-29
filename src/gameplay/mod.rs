use bevy::prelude::*;

mod behavior;
mod catching;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Waiting,
    Catch,
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Waiting)
            .add_system_set(SystemSet::on_enter(GameState::Catch).with_system(catching::init))
            .add_system_set(catching::update_set(GameState::Catch))
            .add_system_set(catching::exit_set(GameState::Catch));
    }
}
