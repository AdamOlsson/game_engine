pub struct CursorMovedEvent {
    x: f64,
    y: f64,
}

impl From<winit::dpi::PhysicalPosition<f64>> for CursorMovedEvent {
    fn from(position: winit::dpi::PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}
