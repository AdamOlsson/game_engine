pub struct CursorMovedEvent {
    pub x: f64,
    pub y: f64,
}

impl From<winit::dpi::PhysicalPosition<f64>> for CursorMovedEvent {
    fn from(position: winit::dpi::PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

impl std::fmt::Display for CursorMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CursorMovedEvent({}, {})", self.x, self.y)
    }
}
