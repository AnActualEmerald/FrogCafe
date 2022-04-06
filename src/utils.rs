use bevy::prelude::{Commands, Component, Entity, Query};

pub fn cleanup_ents<T>(mut commands: Commands, q: Query<Entity, With<T>>)
where
    T: Component,
{
    q.for_each(|e| commands.entity(e).despawn());
}
