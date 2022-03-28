use crate::assets::{CatcherSprite, FlySprite};
use crate::input::MousePos;
use crate::PHYS_SCALE;
use bevy::prelude::*;
use rand::random;

use super::GameState;

#[derive(Component)]
struct Grabber;

static GRABBER_SCALE: f32 = 4.0;

pub fn init(mut commands: Commands, catcher_sprite: Res<CatcherSprite>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: catcher_sprite.0.clone(),
            transform: Transform::default().with_scale(Vec3::splat(GRABBER_SCALE)),
            ..Default::default()
        })
        .insert(Grabber);
}

//put input handling and actual gameplay stuff here
pub fn update_set(state: GameState) -> SystemSet {
    SystemSet::on_update(state)
        .with_system(grabber_movement)
        .with_system(spawn_flies)
}

//despawn relevant entities here
pub fn exit_set(state: GameState) -> SystemSet {
    SystemSet::on_exit(state)
}

//attach grabber to the mouse
fn grabber_movement(mut grabber_q: Query<&mut Transform, With<Grabber>>, m_pos: Res<MousePos>) {
    let grabber_translation = &mut grabber_q.single_mut().translation;
    grabber_translation.x = m_pos.x;
    grabber_translation.y = m_pos.y;
    info!("Cursor pos {:?}", m_pos);
}

fn spawn_flies(
    mut commands: Commands,
    fly_sprite: Res<FlySprite>,
    mut elapsed: Local<f32>,
    time: Res<Time>,
) {
    *elapsed += time.delta_seconds();

    if *elapsed >= 3.0 {
        commands.spawn_bundle(SpriteBundle {
            texture: fly_sprite.0.clone(),
            transform: Transform::from_translation(
                Vec2::splat(*elapsed * random::<f32>()).extend(0.0),
            )
            .with_scale(Vec3::splat(3.0)),
            ..Default::default()
        });
        *elapsed = 0.0;
    }
}
