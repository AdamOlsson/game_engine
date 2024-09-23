use wgpu::util::DeviceExt;

use crate::engine::renderer_engine::{graphics_context::GraphicsContext, util::{create_sampler, create_texture, texture_bind_group_from_texture}, vertex::Vertex};

use super::post_process_filter::{PostProcessFilter, PostProcessFilterBuilder};

pub struct PostProcessPipeline{
    filters: Vec<PostProcessFilter>,
    identity: PostProcessFilter,
}

impl PostProcessPipeline {

    pub fn new(
        g_ctx: &GraphicsContext, pp_ctx: &PostProcessPipelineContext,
    ) -> Self {
        let identity = PostProcessFilterBuilder::identity().build(&g_ctx, &pp_ctx);
        let filters = vec![];
        Self {  filters, identity }

    }

    pub fn add_filters(&mut self, f: &mut Vec<PostProcessFilter>) {
       self.filters.append(f);
    }

    pub fn run(
        &mut self, g_ctx: &GraphicsContext, pp_ctx: &PostProcessPipelineContext,
        texture_handle: &wgpu::Id<wgpu::Texture>,
    ) -> Result<wgpu::Id<wgpu::Texture>,wgpu::SurfaceError> {

        // Texture A contains the output
        let texture_a = pp_ctx.request_texture_by_handle(&texture_handle).unwrap();
        let texture_b = pp_ctx.request_other_texture_by_handle(&texture_handle).unwrap();
        let bind_group_a = pp_ctx.request_bind_group_by_handle(&texture_handle).unwrap();
        let bind_group_b = pp_ctx.request_other_bind_group_by_handle(&texture_handle).unwrap();
        let index_format = pp_ctx.index_format;

        // Ping pong between the two bind_groups
        self.filters.iter_mut().enumerate().for_each(
            |(i, f)| {
                match i % 2 {
                    0 => 
                        f.render(g_ctx, &texture_b, &pp_ctx.vertex_buffer,
                            &pp_ctx.index_buffer, &index_format, &bind_group_a).unwrap(),
                    1 =>
                        f.render(g_ctx, &texture_a, &pp_ctx.vertex_buffer,
                            &pp_ctx.index_buffer, &index_format, &bind_group_b).unwrap(),
                    _ => panic!("How did you get here?..."),
                }
            });
        
        // Return the texture id with the latest result
        let output_texture = match self.filters.len() % 2 {
            0 => pp_ctx.texture_a.global_id(),
            1 => pp_ctx.texture_b.global_id(),
            _ => panic!("How did you get here?..."),
        };

        Ok(output_texture)
    }

    pub fn finalize(
        &mut self, g_ctx: &GraphicsContext, pp_ctx: &PostProcessPipelineContext,
        texture_handle: &wgpu::Id<wgpu::Texture>, surface: &wgpu::SurfaceTexture
    ) -> Result<(),wgpu::SurfaceError> {
        let source = pp_ctx.request_bind_group_by_handle(&texture_handle).unwrap();
        self.identity.render(g_ctx, &surface.texture, &pp_ctx.vertex_buffer,
            &pp_ctx.index_buffer, &pp_ctx.index_format, &source)
    }
}


pub struct PostProcessPipelineContext {
    pub texture_a: wgpu::Texture,
    pub texture_b: wgpu::Texture,
    pub bind_group_a: wgpu::BindGroup,
    pub bind_group_b: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_format: wgpu::IndexFormat,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl PostProcessPipelineContext {
    pub fn new(
        g_ctx: &GraphicsContext, window_size: &winit::dpi::PhysicalSize<u32>
    ) -> Self {
        let (texture_a, bind_group_a, bind_group_layout) = Self::create_texture_bind_group(
            g_ctx, window_size, "a");
        let (texture_b, bind_group_b, _) = Self::create_texture_bind_group(
            g_ctx, window_size, "b");
        let vertices = [
            Vertex { position: [-1.,  1., 0.]},
            Vertex { position: [-1., -1., 0.]},
            Vertex { position: [ 1.,  1., 0.]},
            Vertex { position: [ 1., -1., 0.]},
        ];


        // Miss this and you will spend many hours debugging
        let index_format = wgpu::IndexFormat::Uint16 ;
        let indices: [u16; 6] = [0,1,2,1,3,2];

        let vertex_buffer = g_ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(format!("Post process pipeline vertex buffer").as_str()), 
                contents: bytemuck::cast_slice(&vertices[..]), 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST });

        let index_buffer = g_ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some(format!("Post process pipeline index buffer").as_str()), 
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self {
            bind_group_a, bind_group_b, 
            vertex_buffer, index_buffer, index_format,
            texture_a, texture_b, bind_group_layout }
    }

    pub fn request_texture_by_handle(
        &self, handle: &wgpu::Id<wgpu::Texture>
    ) -> Option<&wgpu::Texture> {
        if *handle == self.texture_a.global_id() {
            return Some(&self.texture_a);
        } else if *handle == self.texture_b.global_id() {
            return Some(&self.texture_b);
        }
        return None; 
    }

    pub fn request_bind_group_by_handle(
        &self, handle: &wgpu::Id<wgpu::Texture>
    ) -> Option<&wgpu::BindGroup> {
        if *handle == self.texture_a.global_id() {
            return Some(&self.bind_group_a);
        } else if *handle == self.texture_b.global_id() {
            return Some(&self.bind_group_b);
        }
        return None; 
    }

    pub fn request_other_texture_by_handle(
        &self, handle: &wgpu::Id<wgpu::Texture>
    ) -> Option<&wgpu::Texture> {
        if *handle == self.texture_a.global_id() {
            return Some(&self.texture_b);
        } else if *handle == self.texture_b.global_id() {
            return Some(&self.texture_a);
        }
        return None; 
    }

    pub fn request_other_bind_group_by_handle(
        &self, handle: &wgpu::Id<wgpu::Texture>
    ) -> Option<&wgpu::BindGroup> {
        if *handle == self.texture_a.global_id() {
            return Some(&self.bind_group_b);
        } else if *handle == self.texture_b.global_id() {
            return Some(&self.bind_group_a);
        }
        return None; 
    }

    pub fn request_texture_handle(&self) -> wgpu::Id<wgpu::Texture> {
        self.texture_a.global_id()
    }

    fn create_texture_bind_group(
        ctx: &GraphicsContext, window_size: &winit::dpi::PhysicalSize<u32>, id: &str 
    ) -> (wgpu::Texture, wgpu::BindGroup, wgpu::BindGroupLayout) {
        let input_texture = create_texture(
            ctx, (window_size.width,window_size.height), Some(&format!("{id} texture")));
        let input_texture_sampler = create_sampler(&ctx.device, Some(&format!("{id} sampler")));
        let (input_texture_bg, input_texture_bg_layout) = texture_bind_group_from_texture(
            &ctx.device, &input_texture_sampler, &input_texture, Some(&format!("{id} texture bind group")));
        (input_texture, input_texture_bg, input_texture_bg_layout)
    }

}
