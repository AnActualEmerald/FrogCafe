use crate::{assets::Sprites, input::MousePos};
use bevy::prelude::*;
use heron::prelude::*;
use std::f32::consts::PI;

use super::{behavior::*, GameState};
use crate::gameplay::{GrabbedEvent, ReleasedEvent};

mod input;

#[derive(Component, Default)]
pub struct Grabber {
    pub grab_point: Vec2,
    grabbed: Option<Entity>,
}

struct FlyTimer(Timer);

const GRABBER_SCALE: f32 = 2.0;

#[derive(PhysicsLayer)]
enum PhysLayers {
    Enabled,
    Disabled,
}

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
                transform: Transform::from_translation(starting_pos.extend(5.)),
                ..Default::default()
            },
            fly_marker: Fly,
            rigid_body: RigidBody::Dynamic,
            collider: CollisionShape::Sphere { radius: rad },
            vel: Velocity::from_linear(starting_vel.extend(0.)),
        }
    }
}

pub fn init(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprites.grabber.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_z((PI) / 4.))
                .with_scale(Vec3::splat(GRABBER_SCALE)),
            ..Default::default()
        })
        .insert(RigidBody::Sensor)
        .insert(Grabber {
            grab_point: Vec2::new(-40., 64. - 30.),
            grabbed: None,
        })
        .insert(Collisions::default())
        .with_children(|parent| {
            parent.spawn_bundle((
                CollisionShape::Cuboid {
                    half_extends: Vec3::new(16., 12., 0.),
                    border_radius: None,
                },
                Transform::from_translation(Vec3::new(0., 64., 0.)),
                GlobalTransform::default(),
            ));
        });

    commands.insert_resource(FlyTimer(Timer::from_seconds(3.0, true)));

    //spawn a floor
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., -(720. / 2.) - 5., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(1280. / 2., 5., 0.),
            border_radius: None,
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., (720. / 2.) + 5., 0.)),
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
            transform: Transform::from_translation(Vec3::new(-(1280. / 2.) - 5., 0., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 720. / 2., 0.),
            border_radius: None,
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new((1280. / 2.) + 5., 0., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 720. / 2., 0.),
            border_radius: None,
        });

    //spawn the jar
    commands.spawn_bundle(SpriteBundle {
        texture: sprites.jar.clone(),
        transform: Transform::from_translation(Vec3::new(
            (1280. / 2.) - 45. * 5.,
            -(720. / 2.) + 32. * 5.,
            10.,
        ))
        .with_scale(Vec3::splat(5.)),
        ..Default::default()
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
        .with_system(grabber_movement.label("grabber_move").after("input"))
        .with_system(fly_behavior)
        .with_system(spawn_flies)
        // .with_system(handle_sensors)
        .with_system(grabbed_behavior.after("grabber_move"))
        .with_system(stunned_behavior)
        .with_system(grab_fly)
        .with_system(release_fly)
        .with_system(input::mouse_buttons)
}

//despawn relevant entities here
pub fn exit_set(state: GameState) -> SystemSet { SystemSet::on_exit(state) }

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

//no real good way that I know of to handle reacting to collision events for a single rigidbody
// fn handle_sensors(
//     mut col_events: EventReader<CollisionEvent>,
//     mut grabbed_ev: EventWriter<GrabbedEvent>,
//     mut released_ev: EventWriter<ReleasedEvent>,

//     grabber_q: Query<Entity, With<Grabber>>,
// ) {
//     let grabber = grabber_q.single();
//     for ev in col_events.iter() {
//         match ev {
//             CollisionEvent::Started(col_a, col_b) => {
//                 let ents = vec![col_a.rigid_body_entity(), col_b.rigid_body_entity()];
//                 if ents.contains(&grabber) {
//                     grabbed_ev.send(GrabbedEvent(ents[0], ents[1]))
//                 }
//             }
//             CollisionEvent::Stopped(col_a, col_b) => {
//                 let ents = vec![col_a.rigid_body_entity(), col_b.rigid_body_entity()];
//                 if ents.contains(&grabber) {
//                     released_ev.send(ReleasedEvent(ents[0], ents[1]))
//                 }
//             }
//         }
//     }
// }

fn grab_fly(mut commands: Commands, mut grabbed_ev: EventReader<GrabbedEvent>) {
    for GrabbedEvent(r) in grabbed_ev.iter() {
        commands
            .entity(*r)
            .insert(super::behavior::Grabbed)
            .insert(CollisionLayers::none());
    }
}

fn release_fly(mut commands: Commands, mut release_ev: EventReader<ReleasedEvent>) {
    for ReleasedEvent(r) in release_ev.iter() {
        commands
            .entity(*r)
            .remove::<super::behavior::Grabbed>()
            .remove::<CollisionLayers>();
    }
}
