use std::mem;
use crate::engine::renderer_engine::vertex::Vertex;
use super::Shape;

pub struct Rectangle {}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct RectangleInstance {
    pub color: [f32; 3],
    pub position: [f32; 3], // TODO: rename to center (also for circle)
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub sprite_coord: [f32; 4],
}

impl Default for RectangleInstance {
    fn default() -> Self {
        RectangleInstance {
            color: [255.0,0.0,0.0], position: [0.0,0.0,0.0], width: 10.0, height: 10.0,
            sprite_coord: [0.0,0.0,1.0,1.0], rotation: 0.0,
        }
    }
}

impl Shape for Rectangle {
    fn id() -> String {
        "Rectangle".to_string()
    }

    fn compute_vertices() -> Vec<Vertex> {
        vec![
            Vertex { position: [-1.0,  1.0, 0.0] }, // top left 
            Vertex { position: [-1.0, -1.0, 0.0] }, // bot left 
            Vertex { position: [ 1.0,  1.0, 0.0] }, // top right 
            Vertex { position: [ 1.0, -1.0, 0.0] }, // bot right 
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
                // Color
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Position
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Rotation
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                // Width
                 wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                // Height
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32,
                },
                // Sprite Coord
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                }

            ],
        }
    }
}
