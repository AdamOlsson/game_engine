use wgpu::util::{ BufferInitDescriptor, DeviceExt};
use crate::engine::renderer_engine::asset::font::Font;
use crate::engine::renderer_engine::asset::Asset;
use crate::engine::renderer_engine::graphics_context::GraphicsContext;
use crate::engine::renderer_engine::shapes::rectangle::Rectangle;
use crate::engine::renderer_engine::util::{create_sampler, create_shader_module, create_texture, write_texture};
use crate::engine::renderer_engine::{shapes::circle::Circle, vertex::Vertex};
use crate::engine::renderer_engine::shapes::Shape;

pub struct RenderPass {
    id: String,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    uniform_buf_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,
}

impl RenderPass {
    pub fn render(
        &mut self, device: &wgpu::Device, target_texture: &wgpu::Texture,
        queue: &wgpu::Queue, instance_buffer: Option<&wgpu::Buffer>, num_indices: u32,
        num_instances: u32, clear_texture: bool,
    ) -> Result<(), wgpu::SurfaceError> {

        let id = self.id.as_str();
        let ce_label = format!("{id} Render Encoder");
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some(ce_label.as_str()),
        };

        let mut command_encoder = 
            device.create_command_encoder(&command_encoder_descriptor);
        
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &target_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            resolve_target: None,
            ops: wgpu::Operations {
                load: match clear_texture {
                    true =>  wgpu::LoadOp::Clear(
                    wgpu::Color {
                        r: 0.0,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    }),
                    false => wgpu::LoadOp::Load,
                },
                store: wgpu::StoreOp::Store,
            },
        };

        {
        let rp_label = format!("{id} Render Pass"); 
        let mut render_pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some(rp_label.as_str()),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
               
                // TODO: I wish to somehow set the bind_groups in a loop and make it possible
                // to have a render pass with and without buffer without any effort
                render_pass.set_bind_group(0, &self.uniform_buf_bind_group, &[]);
                render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

                if let Some(buf) = instance_buffer {
                    render_pass.set_vertex_buffer(1, buf.slice(..));
                }
                
                render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_pipeline(&self.render_pipeline);

                // TODO: There is most likely a way I can merge the two render passes (circle,
                // rect) into one vertex (and index) by using the base_vertex 
                render_pass.draw_indexed(0..num_indices, 0, 0..num_instances);
        }

        queue.submit(Some(command_encoder.finish()));

        Ok(())
    }
}

pub struct RenderPassBuilder {
    id: String,
    shader_path: String,
    shader_label: String,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    instance_buffer_layout: Option<wgpu::VertexBufferLayout<'static>>,
    texture_data: Option<Box<dyn Asset>>,
}

impl RenderPassBuilder {
    
    pub fn circle() -> Self {
        let id = Circle::id();
        let shader_path = include_str!("../shapes/shaders/circle.wgsl").to_string();
        let shader_label = "Circle Shader".to_string();
        let vertices = Circle::compute_vertices();
        let indices = Circle::compute_indices();
        let instance_buffer_layout = Some(Circle::instance_buffer_desc());
        let texture_data = None;
        Self { id, shader_path, shader_label, vertices, indices, instance_buffer_layout, texture_data  }
    }

    pub fn rectangle() -> Self {
        let id = Rectangle::id();
        let shader_path = include_str!("../shapes/shaders/rectangle.wgsl").to_string();
        let shader_label = "Rectangle Shader".to_string();
        let vertices = Rectangle::compute_vertices();
        let indices = Rectangle::compute_indices();
        let instance_buffer_layout = Some(Rectangle::instance_buffer_desc());
        let texture_data = None;
        Self { id, shader_path, shader_label, vertices, indices, instance_buffer_layout, texture_data  }
    }

    pub fn background() -> Self {
        let id = "Background".to_string();
        let shader_path = include_str!("./shaders/background.wgsl").to_string();
        let shader_label = "Background  Shader".to_string();
        let vertices = vec![
            Vertex { position: [-1.,  1., 0.]},
            Vertex { position: [-1., -1., 0.]},
            Vertex { position: [ 1.,  1., 0.]},
            Vertex { position: [ 1., -1., 0.]},
        ];
        let indices = vec![0,1,2,1,3,2];
        let instance_buffer_layout = None;
        let texture_data = None;
        Self { id, shader_path, shader_label, vertices, indices, instance_buffer_layout, texture_data }
    }

    pub fn text() -> Self {
        let id = "Text".to_string();
        let shader_path = include_str!("./shaders/text.wgsl").to_string();
        let shader_label = "Text Shader".to_string();
        let vertices = Rectangle::compute_vertices();
        let indices = Rectangle::compute_indices();
        let instance_buffer_layout = Some(Font::instance_buffer_desc());
        let texture_data = None;
        Self { id, shader_path, shader_label, vertices, indices, instance_buffer_layout, texture_data }
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

        let uniform_buf_bind_group = device.create_bind_group(
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

        (uniform_buffer, uniform_buf_bind_group, uniform_buffer_group_layout)
    }

    fn create_texture_bind_group_from_sprite_sheet(
        device: &wgpu::Device, texture: wgpu::Texture, sampler: wgpu::Sampler, sprite_sheet: &Box<dyn Asset>
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let sprite_data_buffer = device.create_buffer_init(
            &BufferInitDescriptor{
                label: Some("Global render information"),
                contents: bytemuck::cast_slice(&sprite_sheet.specific_data()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });
        
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("texture_bind_group_layout"),
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: sprite_data_buffer.as_entire_binding(),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        (bind_group, layout)
    }

    pub fn texture_data(mut self, tex_data: Box<dyn Asset>) -> Self {
        self.texture_data = Some(tex_data);
        self
    }

    // TODO: Should this also return the instance buffer?
    pub fn build(self, ctx: &GraphicsContext, window_size: &winit::dpi::PhysicalSize<u32>) -> RenderPass {
        let id = self.id;
        let vertex_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some(&self.shader_label),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
        let index_buffer = ctx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some(&self.shader_label),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let (texture_bind_group, texture_bind_group_layout) = match self.texture_data {
            Some(data) => {
                let texture = create_texture(&ctx, data.buffer().dimensions(), Some(format!("{} Sprite Sheet", id.clone()).as_str()));
                write_texture(&ctx, &texture, data.buffer());
                let sampler = create_sampler(&ctx.device);
                Self::create_texture_bind_group_from_sprite_sheet(&ctx.device, texture, sampler, &data)
            }
            _ => todo!(), 
        };
        
        let shader_module = create_shader_module(&ctx.device, self.shader_path);

        let render_targets = [Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let size = [window_size.width as f32, window_size.height as f32];
        let (_buffer, uniform_buf_bind_group, buffer_bind_group_layout) = Self::create_uniform_buffer_init(&ctx.device, &size); 
        let render_pipeline_layout =
            &ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &buffer_bind_group_layout, &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });


        let vertex_buffer_layouts = if let Some(instance_layout) = self.instance_buffer_layout {
            vec![Vertex::desc(), instance_layout ]
        } else { vec![Vertex::desc() ] };
        
        let render_pipeline = ctx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
    
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &vertex_buffer_layouts,
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

        RenderPass {id, vertex_buffer, index_buffer, render_pipeline, uniform_buf_bind_group, texture_bind_group } 
    }
}
