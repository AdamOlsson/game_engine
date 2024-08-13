use winit::{dpi::PhysicalSize, window::Window};
use crate::engine::{Simulation};
use super::{graphics_context::GraphicsContext, gray::gray::Gray, render_pass};

pub struct RenderEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    pub render_pass: render_pass::RenderPass,
    pub size: PhysicalSize<u32>,

    pub pp_gray: Gray,

    pub instance_buffer: wgpu::Buffer,
}

impl <'a> RenderEngine <'a> {
    pub async fn new(window: Window, instance_buffer_len: u32) -> Self {
        let size = window.inner_size();

        let ctx = GraphicsContext::new(window).await;

        let pass_builder = render_pass::RenderPassBuilder::circle();
        let render_pass = pass_builder.build(&ctx.device, &size);
        
        let instance_buffer = ctx.create_buffer(
            "Circle instance buffer", instance_buffer_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let pp_gray = Gray::new(&ctx.device, &size);

        Self { ctx, size, render_pass, instance_buffer, pp_gray }
    }

    pub fn render(
        &mut self, physics_engine: &Box<dyn Simulation>
    ) -> Result<(), wgpu::SurfaceError>{
        let target_texture = &self.pp_gray.texture;

        self.render_pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            &self.instance_buffer, physics_engine.get_num_indices(), 
            physics_engine.get_num_active_instances())?;

        // Post processing
        let output_frame = self.ctx.surface.get_current_texture()?;
        self.pp_gray.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();

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
