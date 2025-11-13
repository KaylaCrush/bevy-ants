use bevy::window::CursorMoved;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::input::ButtonState;
/// Resource that stores the latest mouse position in window/screen coordinates (pixels).
/// None means we haven't received any cursor position yet.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct MousePosition(pub Option<Vec2>);

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct MouseButtons {
    pub left: bool,
    pub right:bool,
}


/// Plugin to register the mouse tracking resource and update system.
pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .init_resource::<MouseButtons>()
            .add_systems(Update, (update_mouse_position,update_mouse_buttons));
    }
}

pub fn update_mouse_position(
    mut cursor_moved_events: MessageReader<CursorMoved>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut mouse_pos: ResMut<MousePosition>,
) {
    if let Some(ev) = cursor_moved_events.read().last() {
        let Ok((camera, camera_transform)) = cameras.single() else {
            return;
        };

        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, ev.position) {
            mouse_pos.0 = Some(world_pos);
        }
    }
}

pub fn update_mouse_buttons(
    mut mouse_button_events: MessageReader<MouseButtonInput>,
    mut mouse_buttons: ResMut<MouseButtons>,
){
    for ev in mouse_button_events.read(){
        if ev.button == MouseButton::Left {
            mouse_buttons.left = (ev.state == ButtonState::Pressed)
        }
        else if ev.button == MouseButton::Right {
            mouse_buttons.right = (ev.state == ButtonState::Pressed)
        }
    }
}
