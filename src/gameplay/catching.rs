use crate::assets::Sprites;
use crate::input::MousePos;
use bevy::prelude::*;
use heron::prelude::*;

use super::{behavior::*, GameState};

#[derive(Component)]
struct Grabber;

struct FlyTimer(Timer);

const GRABBER_SCALE: f32 = 2.0;

#[derive(Bundle)]
struct FlyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    fly_marker: Fly,
    rigid_body: RigidBody,
    collider: CollisionShape,
    vel: Velocity,
}

impl FlyBundle {
    fn new(sprite: Handle<Image>, rad: f32, starting_pos: Vec2, starting_vel: Vec2) -> Self {
        FlyBundle {
            sprite_bundle: SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(starting_pos.extend(0.)),
                ..Default::default()
            },
            fly_marker: Fly,
            rigid_body: RigidBody::Dynamic,
            collider: CollisionShape::Sphere { radius: rad },
            vel: Velocity::from_linear(starting_vel.extend(0.)),
        }
    }
}

pub fn init(mut commands: Commands, sprites: Res<Sprites>, windows: Res<Windows>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprites.grabber.clone(),
            transform: Transform::default().with_scale(Vec3::splat(GRABBER_SCALE)),
            ..Default::default()
        })
        .insert(Grabber);

    commands.insert_resource(FlyTimer(Timer::from_seconds(3.0, true)));

    //spawn a floor
    let w = windows.get_primary().unwrap();
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., -(720. / 2.) - 6., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(1280. / 2., 5., 0.),
            border_radius: None,
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., (720. / 2.) + 6., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(1280. / 2., 5., 0.),
            border_radius: None,
        });

    //spawn a wall
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-(1280. / 2.) - 6., 0., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 720. / 2., 0.),
            border_radius: None,
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new((1280. / 2.) + 6., 0., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 720. / 2., 0.),
            border_radius: None,
        });

    let mut flies = Vec::with_capacity(50);
    for _ in 0..flies.capacity() {
        flies.push(FlyBundle::new(
            sprites.fly.clone(),
            8.,
            Vec2::new(-(1280. / 2.) + 100., -(720. / 2.) + 10.),
            Vec2::new(100., 100.),
        ));
    }

    commands.spawn_batch(flies);
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
}

fn spawn_flies(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut timer: ResMut<FlyTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn_bundle(FlyBundle::new(
            sprites.fly.clone(),
            8.,
            Vec2::new(-(1280. / 2.) + 100., -(720. / 2.) + 10.),
            Vec2::new(100., 100.),
        ));
    }
}
