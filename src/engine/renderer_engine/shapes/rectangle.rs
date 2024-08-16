use std::mem;
use crate::engine::renderer_engine::vertex::Vertex;
use super::Shape;

pub struct Rectangle {}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleInstance {
    pub color: [f32; 3],
    pub position: [f32; 3],
    pub width: f32,
    pub height: f32,
}

impl Shape for Rectangle {
    fn id() -> String {
        "Rectangle".to_string()
    }

    fn compute_vertices() -> Vec<Vertex> {
        vec![
            Vertex { position: [0.0, 1.0, 0.0] }, // top left 
            Vertex { position: [0.0, 0.0, 0.0] }, // bot left 
            Vertex { position: [1.0, 1.0, 0.0] }, // top right 
            Vertex { position: [1.0, 0.0, 0.0] }, // bot right 
        ]
    }

    fn compute_indices() -> Vec<u16> {
        vec![
            0,1,2, // top left
            1,3,2, // bot right
        ]
    }

    fn instance_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RectangleInstance>() as wgpu::BufferAddress,
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
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                }
            ],
        }
    }
}
