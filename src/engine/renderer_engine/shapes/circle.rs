
use crate::engine::renderer_engine::vertex::Vertex;

use super::Shape;

#[allow(dead_code)]
pub struct Circle {
    pub indices: Vec<u16>,
    pub num_indices: u32,
    pub vertices: Vec<Vertex>,
}

impl Shape for Circle { 
    fn id() -> String {
        "Circle".to_string()
    }

    fn compute_vertices() -> Vec<Vertex> {
        let radius = 1.0;
        let x = 0.0;
        let y = 0.0;
        let mut vertices = Vec::new();
        vertices.push(Vertex { position: [x, y, 0.0], color: [1.0, 0.0, 0.0] });
        for i in 0..360 {
            let angle = i as f32 * std::f32::consts::PI / 180.0;
            vertices.push(Vertex {
                position: [x + radius * angle.cos(), y + radius * angle.sin(), 0.0],
                color: [1.0, 0.0, 0.0],
            });
        }
        return vertices;
    }

    fn compute_indices() -> Vec<u16> {
        let mut indices = Vec::new();
        for i in 1..359 {
            indices.push(i as u16);
            indices.push((i + 1) as u16);
            indices.push(0);
        }
        indices.push(359);
        indices.push(1);
        indices.push(0);
        return indices;
    }
}
