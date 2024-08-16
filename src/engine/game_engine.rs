use std::iter::zip;
use winit::keyboard::KeyCode;
use winit::window::Window;
use crate::engine::Simulation;
use crate::engine::renderer_engine::render_engine::RenderEngine;
use super::{physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType}, renderer_engine::shapes::{circle::CircleInstance, rectangle::RectangleInstance}};
use cgmath::{Vector3, Zero};

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

        Self::write_to_rectangle_instance_buffer(&physics_engine, &render_engine);
        Self::write_to_circle_instance_buffer(&physics_engine, &render_engine);
        let _ = render_engine.render_rectangles(&physics_engine, true);
        let _ = render_engine.render_circles(&physics_engine, false);

        let _ = render_engine.post_process();

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

    fn write_to_circle_instance_buffer(physics_engine: &Box<dyn Simulation>, render_engine: &RenderEngine) {
        let bodies = physics_engine.get_bodies();
        let colors = physics_engine.get_colors();
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                match body.body_type { 
                    CollisionBodyType::Circle { radius } => 
                        Some(CircleInstance {
                            position: body.position.into(), 
                            color: (*color).into(), 
                            radius,
                        }),
                    _ => None
                }
        }).collect::<Vec<_>>();

        render_engine.ctx.queue.write_buffer(&render_engine.circle_instance_buffer, 
              0, bytemuck::cast_slice(&instances));
    }

    fn write_to_rectangle_instance_buffer(_physics_engine: &Box<dyn Simulation>, render_engine: &RenderEngine) {
        let bodies = vec![
            CollisionBody::rectangle(
                99, Vector3::zero(), Vector3::zero(), Vector3::zero(), Vector3::zero(), 
                100.0, 100.0),
        ];
        let colors: Vec<Vector3<f32>> = vec![Vector3::new(255.0, 0.0, 0.0),];
        let instances = zip(bodies, colors).filter_map(
            |(body, color)| {
                match body.body_type { 
                    CollisionBodyType::Rectangle{ width, height } => 
                        Some(RectangleInstance {
                            color: color.into(), 
                            position: body.position.into(),
                            width,height
                        }),
                    _ => None
                }
            }).collect::<Vec<_>>();

        render_engine.ctx.queue.write_buffer(&render_engine.rectangle_instance_buffer, 
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
 
        let raw_circle_instances: &[u8] = bytemuck::cast_slice(&instances);
        // FIXME: Dummy data for now
        let raw_rectangle_instances: &[u8] = bytemuck::cast_slice(&[
            RectangleInstance { color: [255.0,0.0,0.0], position: [0.0,0.0,0.0], width: 1.0, height:1.0 }]);
        let render_engine = RenderEngine::new(
            window, raw_circle_instances.len() as u32, raw_rectangle_instances.len() as u32).await;
       
        GameEngine {
            physics_engine, render_engine
        }
    }
}
