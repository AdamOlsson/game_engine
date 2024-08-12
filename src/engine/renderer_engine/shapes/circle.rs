
use crate::engine::renderer_engine::vertex::Vertex;
use std::mem;
use super::Shape;

#[allow(dead_code)]
pub struct Circle {
    pub indices: Vec<u16>,
    pub num_indices: u32,
    pub vertices: Vec<Vertex>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleInstance {
    pub position: [f32; 3],
    pub color: [f32; 3], 
    pub radius: f32,
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
        vertices.push(Vertex { position: [x, y, 0.0] });
        for i in 0..360 {
            let angle = i as f32 * std::f32::consts::PI / 180.0;
            vertices.push(Vertex {
                position: [x + radius * angle.cos(), y + radius * angle.sin(), 0.0] });
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

    fn instance_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<CircleInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                }
            ],
        }
    }
}
