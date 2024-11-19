use super::ElementState;

pub struct MouseInputEvent {
    pub state: ElementState,
    pub button: MouseButton,
}

pub enum MouseButton {
    Unknown,
    Left,
    Middle,
    Right,
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(event: winit::event::MouseButton) -> Self {
        match event {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            _ => MouseButton::Unknown,
        }
    }
}
