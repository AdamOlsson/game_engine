use super::ElementState;

pub struct KeyEvent {
    pub key: Key,
    pub state: ElementState,
    pub repeat: bool,
}

impl From<winit::keyboard::PhysicalKey> for Key {
    fn from(physical_key: winit::keyboard::PhysicalKey) -> Self {
        match physical_key {
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => Key::W,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => Key::A,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => Key::S,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => Key::D,
            _ => Key::Unkown,
        }
    }
}

impl From<winit::event::KeyEvent> for KeyEvent {
    fn from(key_event: winit::event::KeyEvent) -> Self {
        Self {
            key: Key::from(key_event.physical_key),
            state: ElementState::from(key_event.state),
            repeat: key_event.repeat,
        }
    }
}

pub enum Key {
    W,
    A,
    S,
    D,
    Unkown,
}
