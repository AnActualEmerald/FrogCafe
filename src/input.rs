use crate::MainCamera;
use bevy::{input::mouse::MouseMotion, prelude::*};

const MOUSE_SENS: f32 = 10.;

pub struct InputPlugin;

//mouse position in world coords
pub type MousePos = Vec2;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>()
            .add_system(track_mouse.label("input"))
            .add_system(lock_mouse_to_center.after("input"));
    }
}

fn track_mouse(
    mut mouse_pos: ResMut<MousePos>,
    mut move_ev: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    // let mut i = 0;
    for e in move_ev.iter() {
        // i += 1;
        // info!("count  {}", i);
        *mouse_pos += e.delta * Vec2::new(MOUSE_SENS, -MOUSE_SENS) * time.delta_seconds();
    }
}

fn lock_mouse_to_center(mut windows: ResMut<Windows>) {
    let mut wind = windows.get_primary_mut().unwrap();
    if wind.is_focused() && wind.cursor_locked() {
        wind.set_cursor_position(Vec2::new(1280. / 2., 720. / 2.));
    }
}

// fn track_mouse(
//     wnds: Res<Windows>,
//     cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//     mut pos_res: ResMut<MousePos>,
// ) {
//     let (cam, cam_tr) = cam_q.single();
//     let main_window = wnds.get_primary().unwrap();

//     //Thanks to https://bevy-cheatbook.github.io/cookbook/cursor2world.html
//     if let Some(screen_pos) = main_window.cursor_position() {
//         let size = Vec2::new(main_window.width() as f32, main_window.height() as f32);

//         let ndc = (screen_pos / size) * 2.0 - Vec2::ONE;

//         let ndc_to_world = cam_tr.compute_matrix() * cam.projection_matrix.inverse();

//         let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

//         pos_res.x = world_pos.x;
//         pos_res.y = world_pos.y;
//     }
// }
