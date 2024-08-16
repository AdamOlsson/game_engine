use winit::{dpi::PhysicalSize, window::Window};
use crate::engine::Simulation;
use super::{graphics_context::GraphicsContext, gray::gray::Gray, render_pass, shapes::{rectangle::{Rectangle, RectangleInstance}, Shape}};

pub struct RenderEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    pub size: PhysicalSize<u32>,

    pub pp_gray: Gray,

    pub circle_render_pass: render_pass::RenderPass,
    pub circle_instance_buffer: wgpu::Buffer,

    pub rectangle_render_pass: render_pass::RenderPass,
    pub rectangle_instance_buffer: wgpu::Buffer,
}

impl <'a> RenderEngine <'a> {
    pub async fn new(
        window: Window, circle_instance_buffer_len: u32, rectangle_instance_buffer_len: u32

    ) -> Self {
        let size = window.inner_size();

        let ctx = GraphicsContext::new(window).await;

        let circle_pass_builder = render_pass::RenderPassBuilder::circle();
        let circle_render_pass = circle_pass_builder.build(&ctx.device, &size);
        let circle_instance_buffer = ctx.create_buffer(
            "Circle instance buffer", circle_instance_buffer_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

       let rectangle_pass_builder = render_pass::RenderPassBuilder::rectangle();
       let rectangle_render_pass = rectangle_pass_builder.build(&ctx.device, &size);
       let rectangle_instance_buffer = ctx.create_buffer(
            "Rectangle instance buffer", rectangle_instance_buffer_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let pp_gray = Gray::new(&ctx.device, &size);

        Self {
            ctx, size, pp_gray, 
            circle_render_pass, circle_instance_buffer,
            rectangle_render_pass, rectangle_instance_buffer,
        }
    }

    pub fn render_circles(
        &mut self, physics_engine: &Box<dyn Simulation>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let pass = &mut self.circle_render_pass;
        let instance_buffer = &self.circle_instance_buffer;
        let target_texture = &self.pp_gray.texture;
        
        // FIXME: num_indices should no longer come from physics_engine
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            instance_buffer, physics_engine.get_num_indices(), 
            physics_engine.get_num_active_instances(), clear)?;

        return Ok(());
    } 

    pub fn render_rectangles(
        &mut self, _physics_engine: &Box<dyn Simulation>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let pass = &mut self.rectangle_render_pass;
        let instance_buffer = &self.rectangle_instance_buffer;
        let target_texture = &self.pp_gray.texture;
        
        // FIXME: num_indices should no longer come from physics_engine
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            instance_buffer, Rectangle::compute_indices().len() as u32, 1, clear)?;

        return Ok(());
    } 

    pub fn post_process(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output_frame = self.ctx.surface.get_current_texture()?;
        self.pp_gray.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();
        output_frame.present();
        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.ctx.config.width = new_size.width;
        self.ctx.config.height = new_size.height;
        self.ctx.surface.configure(&self.ctx.device, &self.ctx.config);
    }
}
