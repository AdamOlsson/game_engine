use std::iter::zip;

use winit::keyboard::KeyCode;
use winit::window::Window;
use crate::engine::Simulation;
use crate::engine::renderer_engine::Pass;
use crate::engine::renderer_engine::render_pass::RenderPass;
use crate::engine::renderer_engine::instance::Instance;
use crate::engine::renderer_engine::gray::gray::Gray;
use crate::engine::renderer_engine::graphics_context::GraphicsContext;

use super::physics_engine::collision::collision_body::CollisionBodyType;

pub struct GameEngine<'a> {
    pub ctx: GraphicsContext<'a>,
    pass: RenderPass,
    size: winit::dpi::PhysicalSize<u32>,

    physics_engine: Box<dyn Simulation + 'static>,
    
    // Post processing
    pp_gray: Gray,

    instance_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl <'a> GameEngine <'a> {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.ctx.config.width = new_size.width;
        self.ctx.config.height = new_size.height;
        self.ctx.surface.configure(&self.ctx.device, &self.ctx.config);
    }

    pub fn update(&mut self) {
        let simulation = &mut self.physics_engine;
        simulation.update();

        let bodies = simulation.get_bodies();
        let colors = simulation.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                if let CollisionBodyType::Circle { radius } = body.body_type {
                    Some(Instance{
                        position: body.position, color: *color, 
                        radius 
                    }.to_raw())
                } else {
                    None
                }
        }).collect::<Vec<_>>();

        // To prevent writing the static colors every run, we probably can use a global buffer and write 
        // the colors to it once (maybe and then copy it to the instance buffer every frame.)
        self.ctx.queue.write_buffer(&self.instance_buffer, 
             0, bytemuck::cast_slice(&instances));
        
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let simulation = &mut self.physics_engine;
        let target_texture = &self.pp_gray.texture;
        self.pass.draw(&target_texture, &self.ctx.device, &self.ctx.queue,
            &self.vertex_buffer, &self.index_buffer, &self.instance_buffer,
            simulation.get_num_indices(),
            simulation.get_num_active_instances(),
        ).unwrap();

        // Post processing
        let output_frame = self.ctx.surface.get_current_texture()?;
        self.pp_gray.render(&output_frame.texture, &self.ctx.device, &self.ctx.queue).unwrap();

        output_frame.present();

        return Ok(());
    }

    pub fn send_keyboard_input(&mut self, input: KeyCode) {
        match input {
            KeyCode::Space => self.physics_engine.jump(),
            _ => ()
        }
    }
}


pub struct GameEngineBuilder {
    physics_engine: Option<Box<dyn Simulation>>,
}

impl GameEngineBuilder {
    pub fn new() -> Self {
        Self { physics_engine: None }
    }

    pub fn physics_engine(mut self, sim: Box<dyn Simulation>) -> Self {
        self.physics_engine = Some(sim);
        self
    }


    pub async fn build(self, window: Window) -> GameEngine<'static>{
        let simulation = self.physics_engine.unwrap();
        let size = window.inner_size();

        let mut ctx = GraphicsContext::new(window).await;

        let pass = RenderPass::new(&ctx.device, &size);

        let vertex_buffer = ctx.create_buffer(
            "Circle vertex buffer", bytemuck::cast_slice(&simulation.get_vertices()),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST);

        let index_buffer = ctx.create_buffer(
                "Circle index buffer", bytemuck::cast_slice(&simulation.get_indices()),
                wgpu::BufferUsages::INDEX);

        let bodies = simulation.get_bodies();
        let colors = simulation.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                if let CollisionBodyType::Circle { radius } = body.body_type {
                    Some(Instance{
                        position: body.position, 
                        color: *color, 
                        radius: radius / size.width as f32
                    }.to_raw())
                } else {
                    None
                }
        }).collect::<Vec<_>>();

        let instance_buffer = ctx.create_buffer(
            "Circle instance buffer", bytemuck::cast_slice(&instances),
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST);

        let pp_gray = Gray::new(&ctx.device, &size);

        GameEngine {
            ctx, pass, size, instance_buffer,
            vertex_buffer, index_buffer, pp_gray,
            physics_engine: simulation 
        }
    }
}


