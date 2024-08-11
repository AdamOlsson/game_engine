use std::{collections::HashMap, iter::zip};
use winit::keyboard::KeyCode;
use winit::window::Window;
use crate::engine::Simulation;
use crate::engine::renderer_engine::instance::Instance;
use crate::engine::renderer_engine::render_engine::RenderEngine;
use crate::engine::renderer_engine::shapes::Shape;
use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::shapes::circle::Circle};

pub struct GameEngine<'a> {
    physics_engine: Box<dyn Simulation + 'static>,
    // FIXME: Should not be public
    pub render_engine: RenderEngine<'a>,
}

impl <'a> GameEngine <'a> {

    pub fn update(&mut self) {
        let physics_engine = &mut self.physics_engine;
        physics_engine.update();

        let bodies = physics_engine.get_bodies();
        let colors = physics_engine.get_colors();
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
        self.render_engine.ctx.queue.write_buffer(&self.render_engine.instance_buffer, 
             0, bytemuck::cast_slice(&instances));
    }

    // TODO: Rename to something like tick()
    pub fn tick(&mut self) -> Result<(), wgpu::SurfaceError> {
        let physics_engine = &mut self.physics_engine;
        let render_engine = &mut self.render_engine;
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
        let physics_engine = self.physics_engine.unwrap();
        let render_engine = RenderEngine::new(window, &physics_engine).await;
       
        //let mut vertices = HashMap::new();
        //let mut indices = HashMap::new();
        //physics_engine.get_bodies().iter().for_each(
        //    |b| match b.body_type {
        //        CollisionBodyType::Circle { radius: _ } => {
        //            vertices.insert(Circle::id(), Circle::compute_vertices());
        //            indices.insert(Circle::id(), Circle::compute_indices());
        //        }, 
        //        _ => panic!(),
        //    });

        //render_engine.set_vertex_buffer();

        GameEngine {
            physics_engine, render_engine
        }
    }
}
