use crate::engine::renderer_engine::{graphics_context::GraphicsContext, vertex::Vertex};

use super::post_process_pipeline::PostProcessPipelineContext;

pub struct PostProcessFilter {
    label: Option<String>,
    render_pipeline: wgpu::RenderPipeline,
}

impl PostProcessFilter {
   
    pub fn render(
        &mut self, g_ctx: &GraphicsContext, target_texture: &wgpu::Texture,
        vertex_buffer: &wgpu::Buffer, index_buffer: &wgpu::Buffer,
        index_format: &wgpu::IndexFormat, input_texture_bind_group: &wgpu::BindGroup,
    ) -> Result<(), wgpu::SurfaceError> {

        let mut command_encoder = g_ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: self.label.as_deref() });
        {
            let target_texture_view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut render_pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: self.label.as_deref(),
                    color_attachments: &[
                        Some(
                            wgpu::RenderPassColorAttachment {
                                view: &target_texture_view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                            }
                        )],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), *index_format);
            render_pass.set_bind_group(0, input_texture_bind_group, &[]);

            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        g_ctx.queue.submit(Some(command_encoder.finish()));

        Ok(())
    }
}

pub struct PostProcessFilterBuilder {
    id: String,
    shader_path: String,
}

impl PostProcessFilterBuilder {

    pub fn identity() -> PostProcessFilterBuilder {
        let id = "Identity".to_string();
        let shader_path = include_str!("./identity/shaders/identity2.wgsl").to_string();
        Self { id, shader_path, }
    }

    pub fn gray() -> PostProcessFilterBuilder {
        let id = "Gray".to_string();
        let shader_path = include_str!("./gray/shaders/gray2.wgsl").to_string();
        Self { id, shader_path, }
    }

    pub fn build(
        self, g_ctx: &GraphicsContext, pp_ctx: &PostProcessPipelineContext
    ) -> PostProcessFilter {
        let id = self.id;
        let label = Some(format!("{id} post process"));

        let render_shader = g_ctx.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(format!("{id} shader").as_str()), 
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(self.shader_path)),
            }); 

        let pipeline_layout = g_ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(format!("{id} pipeline layout").as_str()),
                bind_group_layouts: &[&pp_ctx.bind_group_layout],
                push_constant_ranges: &[] }
        );

        let render_targets = [Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline = g_ctx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(format!("{id} pipeline").as_str()),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &render_shader,
                    entry_point: "vs_main",
                    buffers: &[ Vertex::desc() ]
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                }, 
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                }, 
                fragment: Some(wgpu::FragmentState {
                    module: &render_shader,
                    entry_point: "fs_main",
                    targets: &render_targets
                }),
                multiview: None
            });

        PostProcessFilter { label, render_pipeline }
    }
}
