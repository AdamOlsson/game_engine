use std::iter::zip;

use winit::{dpi::PhysicalSize, window::Window};

use crate::engine::{physics_engine::{self, collision::collision_body::CollisionBodyType}, Simulation};

use super::{graphics_context::GraphicsContext, gray::gray::Gray, instance::Instance, render_pass::RenderPass, Pass};



pub struct RenderEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    pub pass: RenderPass,
    pub size: PhysicalSize<u32>,

    pub pp_gray: Gray,

    pub instance_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl <'a> RenderEngine <'a> {
    // TODO: Remove the dependency to Simulation trait
    pub async fn new(window: Window, physics_engine: &Box<dyn Simulation>) -> Self {
        let size = window.inner_size();

        let mut ctx = GraphicsContext::new(window).await;

        let pass = RenderPass::new(&ctx.device, &size);

        let vertex_buffer = ctx.create_buffer_init(
            "Circle vertex buffer", bytemuck::cast_slice(&physics_engine.get_vertices()),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST);

        let index_buffer = ctx.create_buffer_init(
                "Circle index buffer", bytemuck::cast_slice(&physics_engine.get_indices()),
                wgpu::BufferUsages::INDEX);

        let bodies = physics_engine.get_bodies();
        let colors = physics_engine.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                if let CollisionBodyType::Circle { radius } = body.body_type {
                    Some(Instance{
                        position: body.position, 
                        color: *color, 
                        radius: radius / size.width as f32
                    }.to_raw())
                } else {
                    None
                }
        }).collect::<Vec<_>>();
        
        let instance_buffer = ctx.create_buffer(
            "Circle instance buffer", (bytemuck::cast_slice(&instances) as &[u8]).len() as u32, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let pp_gray = Gray::new(&ctx.device, &size);

        Self { ctx, size, pass, vertex_buffer, index_buffer, instance_buffer, pp_gray }
    }

    pub fn render(&mut self, physics_engine: &Box<dyn Simulation>) -> Result<(), wgpu::SurfaceError>{
        let target_texture = &self.pp_gray.texture;
        self.pass.draw(&target_texture, &self.ctx.device, &self.ctx.queue,
            &self.vertex_buffer, &self.index_buffer, &self.instance_buffer,
            physics_engine.get_num_indices(),
            physics_engine.get_num_active_instances(),
        ).unwrap();

        // Post processing
        let output_frame = self.ctx.surface.get_current_texture()?;
        self.pp_gray.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();
        
        // render_engine.post_process();

        output_frame.present();
        
        return Ok(());
    } 

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.ctx.config.width = new_size.width;
        self.ctx.config.height = new_size.height;
        self.ctx.surface.configure(&self.ctx.device, &self.ctx.config);
    }
}
