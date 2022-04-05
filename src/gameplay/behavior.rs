use crate::gameplay::catching::Grabber;
use bevy::prelude::*;
use heron::prelude::*;
use rand::random;

///if the fly is grabbed or not
#[derive(Component)]
pub struct Grabbed;

#[derive(Component)]
pub struct Fly;

///act like a fly when it isn't being grabbed
pub fn fly_behavior(mut q: Query<&mut Velocity, (With<Fly>, Without<Grabbed>)>) {
    let scale = 10.;

    for mut tr in q.iter_mut() {
        let diff_x = random::<f32>() * random::<i32>().clamp(-2, 2) as f32;
        let diff_y = random::<f32>() * random::<i32>().clamp(-2, 2) as f32;

        tr.linear.x += diff_x * scale;
        tr.linear.y += diff_y * scale;
        tr.linear = tr.linear.clamp(Vec3::splat(-100.), Vec3::splat(100.));
    }
}

pub fn grabbed_behavior(
    mut q: Query<(&mut Transform, &mut Velocity), With<Grabbed>>,
    grabber_q: Query<(&Grabber, &Transform), Without<Grabbed>>,
) {
    let (grabber, gtr) = grabber_q.single();
    for (mut tr, mut vel) in q.iter_mut() {
        vel.linear = Vec3::ZERO;
        tr.translation.x = grabber.grab_point.x;
        tr.translation.y = grabber.grab_point.y;
        tr.translation += gtr.translation;
    }
}
