use bevy::prelude::*;

pub fn cleanup_ents<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn screen_to_world(
    screen_coords: &Vec2,
    camera: (&Camera, &GlobalTransform),
    window: &Window,
) -> Vec2 {
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    let ndc = (*screen_coords / size) * 2.0 - Vec2::ONE;

    let ndc_to_world = camera.1.compute_matrix() * camera.0.projection_matrix.inverse();

    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}

// pub trait Truncate {
//     type T;
//     pub fn truncate(&self) -> T;
// }

// impl Truncate for Vec3 {
//     type T = Vec2;
//     fn truncate(&self) -> Vec2 { Vec2::new(self.x, self.y) }
// }
