pub mod sat;

#[derive(Debug)]
pub struct Projection {
    pub min: f32,
    pub max: f32,
}

impl Projection {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}
