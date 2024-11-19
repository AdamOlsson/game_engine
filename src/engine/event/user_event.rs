use super::cursor_moved_event::CursorMovedEvent;
use super::key_event::KeyEvent;
use super::mouse_input_event::{MouseButton, MouseInputEvent};
use super::ElementState;

pub enum UserEvent {
    Keyboard(KeyEvent),
    Mouse(MouseInputEvent),
    CursorMoved(CursorMovedEvent),
    CursorLeft,
    CursorEntered,
}

impl From<winit::event::WindowEvent> for UserEvent {
    fn from(event: winit::event::WindowEvent) -> Self {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => UserEvent::Keyboard(KeyEvent::from(event)),
            winit::event::WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => Self::Mouse(MouseInputEvent {
                state: ElementState::from(state),
                button: MouseButton::from(button),
            }),
            //winit::event::WindowEvent::Moved(winit::dpi::PhysicalPosition { x, y }) => todo!(),
            winit::event::WindowEvent::CursorLeft { device_id: _ } => Self::CursorLeft,
            winit::event::WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => Self::CursorMoved(CursorMovedEvent::from(position)),
            winit::event::WindowEvent::CursorEntered { device_id: _ } => Self::CursorEntered,
            _ => panic!("Event {event:?} is not a user input"),
        }
    }
}
