pub enum RenderBodyShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

impl std::fmt::Display for RenderBodyShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderBodyShape::Circle { radius } => write!(f, "Circle({})", radius),
            RenderBodyShape::Rectangle { width, height } => {
                write!(f, "Rectangle({},{})", width, height)
            }
        }
    }
}
