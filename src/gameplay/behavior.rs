use bevy::prelude::*;
use heron::prelude::*;
use rand::random;

#[derive(Component)]
pub struct Fly;

pub fn fly_behavior(mut q: Query<&mut Velocity, With<Fly>>) {
    let scale = 10.;

    for mut tr in q.iter_mut() {
        let diff_x = random::<f32>() * random::<i32>().clamp(-2, 2) as f32;
        let diff_y = random::<f32>() * random::<i32>().clamp(-2, 2) as f32;

        tr.linear.x += diff_x * scale;
        tr.linear.y += diff_y * scale;
        tr.linear = tr.linear.clamp(Vec3::splat(-100.), Vec3::splat(100.));
    }
}