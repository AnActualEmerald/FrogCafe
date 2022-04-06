use bevy::prelude::*;

mod behavior;
mod catching;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Waiting,
    Catch,
}

//--events--//
pub struct GrabbedEvent(Entity);
pub struct ReleasedEvent(Entity);
pub struct StunnedEvent(Entity);
pub struct UnStunnedEvent(Entity);

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Waiting)
            .add_event::<GrabbedEvent>()
            .add_event::<ReleasedEvent>()
            .add_event::<StunnedEvent>()
            .add_event::<UnStunnedEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Catch).with_system(catching::init))
            .add_system_set(catching::update_set(GameState::Catch))
            .add_system_set(catching::exit_set(GameState::Catch));
    }
}
