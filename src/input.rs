use crate::{utils::screen_to_world, MainCamera};
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct InputPlugin;

//mouse position in world coords
pub type MousePos = Vec2;
//mouse delta resource;
#[derive(Default)]
pub struct MouseDelta(pub Vec2);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>()
            .init_resource::<MouseDelta>()
            .add_system_to_stage(CoreStage::PreUpdate, track_mouse.label("input"))
            .add_system_to_stage(CoreStage::PreUpdate, mouse_delta);
    }
}

fn mouse_delta(
    mut mouse_d: ResMut<MouseDelta>,
    mut move_ev: EventReader<MouseMotion>,
    wnds: Res<Windows>,
    cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for e in move_ev.iter() {
        // info!("Mouse delta: {}", e.delta);
        mouse_d.0 = e.delta; //screen_to_world(&e.delta, cam_q.single(), wnds.get_primary().unwrap());
                             // info!("Corrected delta: {}", mouse_d.0);
    }
}

fn lock_mouse_to_center(mut windows: ResMut<Windows>) {
    let mut wind = windows.get_primary_mut().unwrap();
    if wind.is_focused() && wind.cursor_locked() {
        wind.set_cursor_position(Vec2::new(1280. / 2., 720. / 2.));
    }
}

fn track_mouse(
    wnds: Res<Windows>,
    cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut pos_res: ResMut<MousePos>,
) {
    let main_window = wnds.get_primary().unwrap();

    //Thanks to https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    if let Some(screen_pos) = main_window.cursor_position() {
        *pos_res = screen_to_world(&screen_pos, cam_q.single(), main_window);
    }
}
