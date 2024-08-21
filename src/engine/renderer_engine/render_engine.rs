use winit::{dpi::PhysicalSize, window::Window};
use super::{graphics_context::GraphicsContext, gray::gray::Gray, identity::identity::Identity, render_pass, shapes::{circle::Circle, rectangle::Rectangle, Shape}};

pub struct RenderEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    pub size: PhysicalSize<u32>,

    pub pp_gray: Option<Gray>,
    pp_identity: Identity,

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
        
        // TODO: Add feature to apply post process in sequence
        //let pp_gray = Some(Gray::new(&ctx.device, &size));
        let pp_gray = None; 
        let pp_identity = Identity::new(&ctx.device, &size);

        Self {
            ctx, size, 
            pp_gray, pp_identity,
            circle_render_pass, circle_instance_buffer,
            rectangle_render_pass, rectangle_instance_buffer,
        }
    }

    pub fn render_circles(
        &mut self, num_instances: u32, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let pass = &mut self.circle_render_pass;
        let instance_buffer = &self.circle_instance_buffer;
        let target_texture = if let Some(tex) = &self.pp_gray { &tex.texture } else { &self.pp_identity.texture };

       
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            instance_buffer, Circle::compute_indices().len() as u32, num_instances, clear)?;

        return Ok(());
    } 

    pub fn render_rectangles(
        &mut self, num_instances: u32, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let pass = &mut self.rectangle_render_pass;
        let instance_buffer = &self.rectangle_instance_buffer;
        let target_texture = if let Some(tex) = &self.pp_gray { &tex.texture } else { &self.pp_identity.texture };
        
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            instance_buffer, Rectangle::compute_indices().len() as u32, num_instances, clear)?;

        return Ok(());
    } 

    pub fn post_process(&mut self) -> Result<(), wgpu::SurfaceError>{
        let output_frame = self.ctx.surface.get_current_texture()?;
        
        if let Some(gray) = &mut self.pp_gray {
            gray.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();
        } else {
            self.pp_identity.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();
        }
        
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
