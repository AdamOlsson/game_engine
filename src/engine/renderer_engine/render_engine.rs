use image::{Rgba, RgbaImage};
use winit::{dpi::PhysicalSize, window::Window};
use crate::engine::physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType};

use super::{graphics_context::GraphicsContext, gray::gray::Gray, identity::identity::Identity, render_pass, shapes::{circle::{Circle, CircleInstance}, rectangle::{Rectangle, RectangleInstance}, Shape}, sprite_sheet::SpriteSheet, util};

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
    sprite_sheet: Option<SpriteSheet>
}

impl <'a> RenderEngineBuilder {
    pub fn new() -> Self {
        Self { circ_instance_buf_len: 0,rect_instance_buf_len: 0, sprite_sheet: None }
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

    pub fn texture(mut self, tex: SpriteSheet) -> Self {
        self.sprite_sheet = Some(tex);
        self
    }

    pub fn build(self,
        ctx: GraphicsContext<'a>,
        window_size: PhysicalSize<u32>,
    ) -> RenderEngine<'a> {

        let sprite_sheet = match self.sprite_sheet{
            Some(b) => b,
            None => SpriteSheet::default(),
        };

        let circle_render_pass = render_pass::RenderPassBuilder::circle()
            .sprite_sheet(sprite_sheet.clone())
            .build(&ctx, &window_size);
        let circle_instance_buffer = ctx.create_buffer(
            "Circle instance buffer", self.circ_instance_buf_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let rectangle_render_pass = render_pass::RenderPassBuilder::rectangle()
            .sprite_sheet(sprite_sheet)
            .build(&ctx, &window_size);
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

