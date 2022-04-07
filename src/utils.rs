use bevy::prelude::*;

pub fn cleanup_ents<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.for_each(|e| commands.entity(e).despawn_recursive());
}
