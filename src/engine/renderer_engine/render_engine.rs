use winit::dpi::PhysicalSize;
use crate::engine::physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType};

use super::{asset::{background::Background, font::{Font, FontInstance}}, graphics_context::GraphicsContext, post_process::{post_process_filter::PostProcessFilterBuilder, post_process_pipeline::{PostProcessPipeline, PostProcessPipelineContext}}, render_pass, shapes::{circle::{Circle, CircleInstance}, rectangle::{Rectangle, RectangleInstance}, Shape}};

use crate::engine::renderer_engine::asset::sprite_sheet::SpriteSheet;

pub struct RenderEngineControl<'a> {
    pub g_ctx: GraphicsContext<'a>,
    window_size: PhysicalSize<u32>,

    //pp_ctx: PostProcessPipelineContext,
    pub pp_ctx: PostProcessPipelineContext,

    post_process_pipeline: PostProcessPipeline,

    background_render_pass: Option<render_pass::render_pass::RenderPass>,

    text_render_pass: Option<render_pass::render_pass::RenderPass>,
    text_instance_buf: Option<wgpu::Buffer>,

    circle_render_pass: render_pass::render_pass::RenderPass,
    pub circle_instance_buffer: wgpu::Buffer,

    rectangle_render_pass: render_pass::render_pass::RenderPass,
    pub rectangle_instance_buffer: wgpu::Buffer,
}

impl <'a> RenderEngineControl <'a> {
    pub fn render_background(
        &mut self, 
        texture_handle: &wgpu::Id<wgpu::Texture>,
    ) -> Result<(), wgpu::SurfaceError> {
        let target_texture = self.pp_ctx.request_texture_by_handle(&texture_handle).unwrap();
        let num_indices = 6;
        if let Some(pass) = &mut self.background_render_pass {
            pass.render(&self.g_ctx.device, target_texture, &self.g_ctx.queue, None, num_indices, 1, true)?;
        } else {
            panic!("Background not set");
        }
        return Ok(());
    }

    pub fn render_circles(
        &mut self, texture_handle: &wgpu::Id<wgpu::Texture>, 
        instances: &Vec<CircleInstance>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let buf = &self.circle_instance_buffer;
        let indices = Circle::compute_indices();
        let pass = &mut self.circle_render_pass;
        let num_instances = instances.len();
        let target_texture = self.pp_ctx.request_texture_by_handle(&texture_handle).unwrap();
        self.g_ctx.queue.write_buffer(&buf, 
              0, bytemuck::cast_slice(&instances));

        pass.render(&self.g_ctx.device, target_texture, &self.g_ctx.queue,
            Some(buf), indices.len() as u32, num_instances as u32, clear)?;

        return Ok(());
    } 

    pub fn render_rectangles(
        &mut self, texture_handle: &wgpu::Id<wgpu::Texture>, 
        instances: &Vec<RectangleInstance>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let buf = &self.rectangle_instance_buffer;
        let indices = Rectangle::compute_indices();
        let pass = &mut self.rectangle_render_pass;
        let num_instances = instances.len();
        let target_texture = self.pp_ctx.request_texture_by_handle(&texture_handle).unwrap();
        self.g_ctx.queue.write_buffer(&buf, 
              0, bytemuck::cast_slice(&instances));

        pass.render(&self.g_ctx.device, target_texture, &self.g_ctx.queue,
            Some(buf), indices.len() as u32, num_instances as u32, clear)?;

        return Ok(());
    } 

    pub fn render_text(
        &mut self, texture_handle: &wgpu::Id<wgpu::Texture>, 
        text: Vec<FontInstance>, clear: bool
    ) -> Result<(), wgpu::SurfaceError>{
        let pass = match &mut self.text_render_pass {
            None => panic!("No font is set"),
            Some(p) => p,
        };

        if let Some(buf) = &self.text_instance_buf {
            let target_texture = self.pp_ctx.request_texture_by_handle(&texture_handle).unwrap();
            let indices = Rectangle::compute_indices();
            let num_instances = text.len();

            self.g_ctx.queue.write_buffer(&buf, 0, bytemuck::cast_slice(&text));

            pass.render(&self.g_ctx.device, target_texture, &self.g_ctx.queue,
                Some(buf), indices.len() as u32, num_instances as u32, clear)?;
        }

        return Ok(());
    }

    pub fn post_process(
        &mut self, texture_handle: &wgpu::Id<wgpu::Texture>,
    ) -> Result<wgpu::Id<wgpu::Texture>, wgpu::SurfaceError>{
        self.post_process_pipeline.run(&self.g_ctx, &self.pp_ctx, &texture_handle)
    }

    pub fn present(
        &mut self, texture_handle: &wgpu::Id<wgpu::Texture>, 
    ) -> Result<(), wgpu::SurfaceError>{
        let surface = self.g_ctx.surface.get_current_texture().unwrap();
        let _ = self.post_process_pipeline.finalize(
            &self.g_ctx, &self.pp_ctx, &texture_handle, &surface);
        surface.present();
        Ok(())
    }

    pub fn request_texture_handle(&mut self) -> wgpu::Id<wgpu::Texture> {
        self.pp_ctx.request_texture_handle()
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.window_size = new_size;
        self.g_ctx.config.width = new_size.width;
        self.g_ctx.config.height = new_size.height;
        self.g_ctx.surface.configure(&self.g_ctx.device, &self.g_ctx.config);
    }
}

pub struct RenderEngineControlBuilder {
    circ_instance_buf_len: u32,
    rect_instance_buf_len: u32,
    sprite_sheet: Option<SpriteSheet>,
    background: Option<Background>,
    font: Option<Font>,
    use_gray_pp: bool,
}

impl <'a> RenderEngineControlBuilder {
    pub fn new() -> Self {
        Self { circ_instance_buf_len: 0,rect_instance_buf_len: 0, sprite_sheet: None, 
            background: None, font: None, use_gray_pp: false }
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

    pub fn sprite_sheet(mut self, tex: SpriteSheet) -> Self {
        self.sprite_sheet = Some(tex);
        self
    }

    pub fn background(mut self, background: Background) -> Self {
        self.background = Some(background);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn use_gray_pp(mut self) -> Self {
        self.use_gray_pp = true;
        self
    }

    pub fn build(self,
        g_ctx: GraphicsContext<'a>,
        window_size: PhysicalSize<u32>,
    ) -> RenderEngineControl<'a> {

        let sprite_sheet = match self.sprite_sheet{
            Some(b) => b,
            None => SpriteSheet::default(),
        };
        
        let (text_render_pass, text_instance_buf) = if let Some(f) = self.font {
            let pass = Some(render_pass::render_pass::RenderPassBuilder::text()
                .texture_data(Box::new(f))
                .build(&g_ctx, &window_size));
            
            let buf = Some(g_ctx.create_buffer("Text instance buffer", 1024,
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false));

            (pass, buf)
        } else {
            (None, None) 
        };

        let background_render_pass = if let Some(bg) = self.background {
            Some(render_pass::render_pass::RenderPassBuilder::background()
                .texture_data(Box::new(bg))
                .build(&g_ctx,&window_size))
        } else { None };
       
        let circle_render_pass = render_pass::render_pass::RenderPassBuilder::circle()
            .texture_data(Box::new(sprite_sheet.clone()))
            .build(&g_ctx, &window_size);
        let circle_instance_buffer = g_ctx.create_buffer(
            "Circle instance buffer", self.circ_instance_buf_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);

        let rectangle_render_pass = render_pass::render_pass::RenderPassBuilder::rectangle()
            .texture_data(Box::new(sprite_sheet.clone()))
            .build(&g_ctx, &window_size);
       let rectangle_instance_buffer = g_ctx.create_buffer(
            "Rectangle instance buffer", self.rect_instance_buf_len, 
             wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, false);
        
        let pp_ctx = PostProcessPipelineContext::new(&g_ctx, &window_size);
        let gray = PostProcessFilterBuilder::gray().build(&g_ctx, &pp_ctx);
        let mut post_process_pipeline = PostProcessPipeline::new(&g_ctx, &pp_ctx);
        post_process_pipeline.add_filters(&mut vec![gray]);

        RenderEngineControl { 
            g_ctx, pp_ctx, window_size, 
            background_render_pass,
            circle_render_pass, circle_instance_buffer,
            rectangle_render_pass, rectangle_instance_buffer,
            text_render_pass, text_instance_buf, post_process_pipeline,
        }
    }
}

