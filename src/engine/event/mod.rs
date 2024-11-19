pub mod cursor_moved_event;
pub mod key_event;
pub mod mouse_input_event;
pub mod user_event;

pub enum ElementState {
    Pressed,
    Released,
}

impl From<winit::event::ElementState> for ElementState {
    fn from(state: winit::event::ElementState) -> Self {
        match state {
            winit::event::ElementState::Pressed => ElementState::Pressed,
            winit::event::ElementState::Released => ElementState::Released,
        }
    }
}
