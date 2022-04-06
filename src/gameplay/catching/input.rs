use super::{Grabbed, GrabbedEvent, Grabber, ReleasedEvent};
use bevy::prelude::*;
use heron::prelude::*;

pub fn mouse_buttons(
    mut btns: Res<Input<MouseButton>>,
    mut grab_ev: EventWriter<GrabbedEvent>,
    mut release_ev: EventWriter<ReleasedEvent>,
    mut grabber_q: Query<&mut Collisions, With<Grabber>>,
    mut grabbed_q: Query<Entity, With<Grabbed>>,
) {
    let grabber_col = grabber_q.single_mut();
    if btns.just_pressed(MouseButton::Left) {
        grab_ev.send_batch(grabber_col.iter().map(|e| GrabbedEvent(e)));
    }
    if btns.just_released(MouseButton::Left) {
        release_ev.send_batch(grabbed_q.iter().map(|e| ReleasedEvent(e)));
    }
}
