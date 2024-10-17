extern crate game_engine;


use game_engine::engine::renderer_engine::graphics_context::GraphicsContext;
use game_engine::engine::renderer_engine::util;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBody;
use game_engine::engine::PhysicsEngine;
use wgpu::util::DeviceExt;
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;


pub struct MainSimulation {
    bodies: Vec<RigidBody>,
}

impl MainSimulation{
    pub fn new() -> Self {
        let bodies = vec![];
        Self { bodies }
    }
}

impl PhysicsEngine for MainSimulation {

    fn update(&mut self) {}

    fn get_bodies(&self) -> &Vec<RigidBody> {
        &self.bodies
    }
} 

enum CustomEvent {
    ServerTick,
    ClientRender,
}

async fn run_compute(input: &Vec<Vec<u32>>) {
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();
    let window =  WindowBuilder::new().build(&event_loop).unwrap();
    let ctx = GraphicsContext::new(window);
   
    let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    has_dynamic_offset: false,
                    //min_binding_size: Some(NonZeroU64::new(1).unwrap()),
                    min_binding_size: None, 
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                },
            },
        ],
    });
    let shader_path = include_str!("compute_shader.wgsl").to_string();
    let shader_module = util::create_shader_module(&ctx.device, shader_path); 
    
    let input_flat: Vec<u32> = input.concat();

    let input_u8 = bytemuck::cast_slice(&input_flat[..]);

    let readback_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input_u8.len() as wgpu::BufferAddress,
        // Can be read to the CPU, and can be copied from the shader's storage buffer
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let storage_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage buffer"),
        contents: &input_u8,
        usage: wgpu::BufferUsages::STORAGE // Storage address space I assume
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "cs_main",
    });


    let mut command_encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Command encoder")});
    {
        let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute pass"), timestamp_writes: None});
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.set_pipeline(&compute_pipeline);
        //compute_pass.dispatch_workgroups(input.len() as u32, 1, 1);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }
    
    command_encoder.copy_buffer_to_buffer(&storage_buffer, 0, &readback_buffer, 0, input_u8.len() as wgpu::BufferAddress);
    
    ctx.queue.submit(Some(command_encoder.finish()));
    let buffer_slice = readback_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {});
    ctx.device.poll(wgpu::Maintain::Wait);


    let output = buffer_slice
        .get_mapped_range()
        .chunks_exact(4)
        .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
        .collect::<Vec<_>>();

    let chunks: Vec<Vec<u32>> = output.chunks(input[0].len()).map(|c|c.to_vec()).collect();
    println!("Output: ");
    chunks.iter().for_each(|c| println!("{:?}", c));
    
}

pub fn main() {
    let data = vec![
        vec![4,2,4,1,2,3,4],
        vec![8,3,2,5,7,1,2],
    ];
    pollster::block_on(run_compute(&data));
}
