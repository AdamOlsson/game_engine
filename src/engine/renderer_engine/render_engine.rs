use image::{Rgba, RgbaImage};
use winit::{dpi::PhysicalSize, window::Window};
use crate::engine::physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType};

use super::{graphics_context::GraphicsContext, gray::gray::Gray, identity::identity::Identity, render_pass, shapes::{circle::{Circle, CircleInstance}, rectangle::{Rectangle, RectangleInstance}, Shape}, util};

pub struct RenderEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    window_size: PhysicalSize<u32>,

    pp_gray: Option<Gray>,
    pp_identity: Identity,

    circle_render_pass: render_pass::RenderPass,
    pub circle_instance_buffer: wgpu::Buffer,

    rectangle_render_pass: render_pass::RenderPass,
    pub rectangle_instance_buffer: wgpu::Buffer,
}

impl <'a> RenderEngine <'a> {
    pub fn render_circles(
        &mut self, instances: &Vec<CircleInstance>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let buf = &self.circle_instance_buffer;
        let indices = Circle::compute_indices();
        let pass = &mut self.circle_render_pass;
        let num_instances = instances.len();

        self.ctx.queue.write_buffer(&buf, 
              0, bytemuck::cast_slice(&instances));

        let target_texture = if let Some(tex) = &self.pp_gray { &tex.texture } else { &self.pp_identity.texture };
        
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            buf, indices.len() as u32, num_instances as u32, clear)?;

        return Ok(());
    } 

    pub fn render_rectangles(
        &mut self, instances: &Vec<RectangleInstance>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let buf = &self.rectangle_instance_buffer;
        let indices = Rectangle::compute_indices();
        let pass = &mut self.rectangle_render_pass;
        let num_instances = instances.len();
        
        // TODO: Start looking at how a user would work with textures
        //assert!(
        //    instances.iter()
        //    .map(
        //        |i| i.texture_coord.is_empty() ||
        //            i.texture_coord.len() == indices.len())
        //    .collect::<Vec<bool>>().all());

        self.ctx.queue.write_buffer(&buf, 
              0, bytemuck::cast_slice(&instances));

        let target_texture = if let Some(tex) = &self.pp_gray { &tex.texture } else { &self.pp_identity.texture };
        
        pass.render(&self.ctx.device, &target_texture, &self.ctx.queue,
            buf, indices.len() as u32, num_instances as u32, clear)?;

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
        self.window_size = new_size;
        self.ctx.config.width = new_size.width;
        self.ctx.config.height = new_size.height;
        self.ctx.surface.configure(&self.ctx.device, &self.ctx.config);
    }
}

pub struct RenderEngineBuilder {
    circ_instance_buf_len: u32,
    rect_instance_buf_len: u32,
    img_buffer: Option<image::ImageBuffer<Rgba<u8>, Vec<u8>>>
}
impl <'a> RenderEngineBuilder {
    pub fn new() -> Self {
        Self { circ_instance_buf_len: 0,rect_instance_buf_len: 0, img_buffer: None }
    }

    pub fn bodies(mut self, bodies: &Vec<CollisionBody>) -> Self {
        let circle_count: u32 = bodies.iter()
            .fold(0, |acc, b| match b.body_type {
                CollisionBodyType::Circle{ .. } => acc + 1,
                _ => acc,
            }); 
        let default_circle = CircleInstance::default();
        let raw_circle_instance = bytemuck::bytes_of(&default_circle);
        let circle_instance_buffer_len = (raw_circle_instance.len() as u32)*circle_count;

        let rect_count: u32 = bodies.iter()
            .fold(0, |acc, b| match b.body_type {
                CollisionBodyType::Rectangle{ .. } => acc + 1,
                _ => acc,
            }); 
        let default_rect = RectangleInstance::default();
        let raw_rect_instance = bytemuck::bytes_of(&default_rect);
        let rect_instance_buffer_len = (raw_rect_instance.len() as u32)*rect_count;

        self.circ_instance_buf_len = circle_instance_buffer_len;
        self.rect_instance_buf_len = rect_instance_buffer_len;
        self
    }

    pub fn texture(mut self, tex: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        self.img_buffer = Some(tex);
        self
    }

    pub fn build(self,
        ctx: GraphicsContext<'a>,
        window_size: PhysicalSize<u32>,
    ) -> RenderEngine<'a> {

        let img_buf = match self.img_buffer {
            Some(b) => b,
            None => RgbaImage::new(1,1),
        };

        let dimensions = &img_buf.dimensions();
        let circ_texture = util::create_texture(&ctx, dimensions, Some("Rectangle Texture"));
        let rect_texture = util::create_texture(&ctx, dimensions, Some("Rectangle Texture"));
        util::write_texture(&ctx, &circ_texture, &img_buf);
        util::write_texture(&ctx, &rect_texture, &img_buf);
        let circ_sampler = util::create_sampler(&ctx.device);
        let rect_sampler = util::create_sampler(&ctx.device);

        // TODO: Should we create a new render pass for textures or include texture
        // in the existing ones?
        // - I will use the existing ones.
        // TODO: How should I pass the texture coordinates to the shader?
        // - We only provide a cell number for the texture inside the instance data and compute 
        // texture sample coordinate inside the shader.
        // TODO: How do I handle that a instance has no texture
        // - A cell id of 0 means do not sample texture. 
        let circle_render_pass = render_pass::RenderPassBuilder::circle()
            .sampler(circ_sampler)
            .texture(circ_texture)
            .build(&ctx.device, &window_size);
        let circle_instance_buffer = ctx.create_buffer(
            "Circle instance buffer", self.circ_instance_buf_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let rectangle_render_pass = render_pass::RenderPassBuilder::rectangle()
            .sampler(rect_sampler)
            .texture(rect_texture)
            .build(&ctx.device, &window_size);
       let rectangle_instance_buffer = ctx.create_buffer(
            "Rectangle instance buffer", self.rect_instance_buf_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);
        
        // TODO: Add feature to apply post process in sequence
        //let pp_gray = Some(Gray::new(&ctx.device, &size));
        let pp_gray = None; 
        let pp_identity = Identity::new(&ctx.device, &window_size);
            
        RenderEngine { 
            ctx, window_size, 
            pp_gray, pp_identity,
            circle_render_pass, circle_instance_buffer,
            rectangle_render_pass, rectangle_instance_buffer,

        }
    }
}

