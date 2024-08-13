use std::iter::zip;
use winit::keyboard::KeyCode;
use winit::window::Window;
use crate::engine::Simulation;
use crate::engine::renderer_engine::render_engine::RenderEngine;
use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::shapes::circle::CircleInstance};

pub struct GameEngine<'a> {
    physics_engine: Box<dyn Simulation + 'static>,
    render_engine: RenderEngine<'a>,
}

impl <'a> GameEngine <'a> {

    pub fn update(&mut self) {
        let physics_engine = &mut self.physics_engine;
        physics_engine.update();
    }

    pub fn tick(&mut self) -> Result<(), wgpu::SurfaceError> {
        let physics_engine = &mut self.physics_engine;
        let render_engine = &mut self.render_engine;
        
        Self::write_to_instance_buffer(&physics_engine, &render_engine);
        let _ = render_engine.render(&physics_engine);
        return Ok(());
    }

    pub fn send_keyboard_input(&mut self, input: KeyCode) {
        match input {
            KeyCode::Space => self.physics_engine.jump(),
            _ => ()
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.render_engine.resize(new_size);
    }

    fn write_to_instance_buffer(physics_engine: &Box<dyn Simulation>, render_engine: &RenderEngine) {
        let bodies = physics_engine.get_bodies();
        let colors = physics_engine.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                if let CollisionBodyType::Circle { radius } = body.body_type {
                    Some(CircleInstance {
                        position: body.position.into(), 
                        color: (*color).into(), 
                        radius,
                    })
                } else {
                    None
                }
        }).collect::<Vec<_>>();

        render_engine.ctx.queue.write_buffer(&render_engine.instance_buffer, 
             0, bytemuck::cast_slice(&instances));
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
        let size = window.inner_size();
        let physics_engine = self.physics_engine.unwrap();

        let bodies = physics_engine.get_bodies();
        let colors = physics_engine.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                if let CollisionBodyType::Circle { radius } = body.body_type {
                    Some(CircleInstance {
                        position: body.position.into(), 
                        color: (*color).into(), 
                        radius: radius / size.width as f32
                    })
                } else {
                    None
                }
        }).collect::<Vec<_>>();
 
        let raw_instances: &[u8]= bytemuck::cast_slice(&instances);
        let render_engine = RenderEngine::new(window, raw_instances.len() as u32).await;
       
        GameEngine {
            physics_engine, render_engine
        }
    }
}
