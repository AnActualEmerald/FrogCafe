use crate::assets::Sprites;
use crate::input::MousePos;
use bevy::prelude::*;
use heron::prelude::*;

use super::{behavior::*, GameState};

#[derive(Component)]
struct Grabber;

struct FlyTimer(Timer);

static GRABBER_SCALE: f32 = 4.0;

pub fn init(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprites.grabber.clone(),
            transform: Transform::default().with_scale(Vec3::splat(GRABBER_SCALE)),
            ..Default::default()
        })
        .insert(Grabber);

    commands.insert_resource(FlyTimer(Timer::from_seconds(3.0, true)));
}

//put input handling and actual gameplay stuff here
pub fn update_set(state: GameState) -> SystemSet {
    SystemSet::on_update(state)
        .with_system(grabber_movement)
        .with_system(fly_behavior)
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
    sprites: Res<Sprites>,
    mut timer: ResMut<FlyTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprites.fly.clone(),
                ..Default::default()
            })
            .insert(Fly)
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 8. })
            .insert(Velocity::from_linear(Vec3::default()));
    }
}
