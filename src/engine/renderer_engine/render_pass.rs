use wgpu::util::{BufferInitDescriptor, DeviceExt};
use super::{shapes::{Shape}, vertex::Vertex};

pub struct RenderPass {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    buffer_bind_group: wgpu::BindGroup,
}

impl RenderPass {
    pub fn render(
        &mut self, device: &wgpu::Device, target_texture: &wgpu::Texture,
        queue: &wgpu::Queue, instance_buffer: &wgpu::Buffer, num_indices: u32,
        num_instances: u32,
    ) -> Result<(), wgpu::SurfaceError> {
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let mut command_encoder = 
            device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &target_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

                render_pass.set_bind_group(0, &self.buffer_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                
                render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.set_pipeline(&self.render_pipeline);
                
                render_pass.draw_indexed(0..num_indices, 0, 0..num_instances);
        }

        queue.submit(Some(command_encoder.finish()));

        Ok(())
    }
}

pub struct RenderPassBuilder {
    shader_path: String,
    shader_label: String,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    instance_buffer_layout: wgpu::VertexBufferLayout<'static>,
}

impl RenderPassBuilder {
    
    pub fn circle() -> Self {
        let shader_path = include_str!("shapes/shaders/circle.wgsl").to_string();
        let shader_label = "Circle Shader".to_string();
        let vertices = super::shapes::circle::Circle::compute_vertices();
        let indices = super::shapes::circle::Circle::compute_indices();
        let instance_buffer_layout = super::shapes::circle::Circle::instance_buffer_desc();
        Self { shader_path, shader_label, vertices, indices, instance_buffer_layout }
    }

    //pub fn rectangle() -> Self {
    //    Self {}
    //}

    fn create_shader_module(device: &wgpu::Device, path: String) -> wgpu::ShaderModule{
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&path.clone()),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(path)),
        })
    }

    fn create_uniform_buffer_init(
        device: &wgpu::Device, data: &[f32]
    ) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
        let uniform_buffer = device.create_buffer_init(
            &BufferInitDescriptor{
                label: Some("Global render information"),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let uniform_buffer_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { 
                label: Some("Global render buffer layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ]
            }
        );

        let uniform_buffer_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Global render information bind group"),
                layout: &uniform_buffer_group_layout, 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ]
            }
        );

        (uniform_buffer, uniform_buffer_bind_group, uniform_buffer_group_layout)
    }



    pub fn build(self, device: &wgpu::Device, window_size: &winit::dpi::PhysicalSize<u32>) -> RenderPass {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some(&self.shader_label),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some(&self.shader_label),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        let shader_module = Self::create_shader_module(device, self.shader_path);

        let render_targets = [Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let size = [window_size.width as f32, window_size.height as f32];
        let (_buffer, buffer_bind_group, buffer_bind_group_layout) = Self::create_uniform_buffer_init(&device, &size); 
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &buffer_bind_group_layout
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
    
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc(), self.instance_buffer_layout ], 
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
    
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &render_targets,
                }),
    
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        RenderPass { vertex_buffer, index_buffer, render_pipeline, buffer_bind_group } 
    }
}
