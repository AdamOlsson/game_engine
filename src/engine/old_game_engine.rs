use winit::keyboard::KeyCode;
use crate::engine::Simulation;
use crate::engine::renderer_engine::render_engine::RenderEngine;
use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::shapes::{circle::CircleInstance, rectangle::RectangleInstance}};

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

        let num_rect_instances = Self::write_to_rectangle_instance_buffer(&physics_engine, &render_engine);
        let num_circle_instances = Self::write_to_circle_instance_buffer(&physics_engine, &render_engine);
        let _ = render_engine.render_rectangles(num_rect_instances as u32, true);
        let _ = render_engine.render_circles(num_circle_instances as u32, false);

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

    fn write_to_circle_instance_buffer(physics_engine: &Box<dyn Simulation>, render_engine: &RenderEngine) -> usize {
        let bodies = physics_engine.get_bodies();
        let instances = bodies.iter().filter_map(
            |body| {
                match body.body_type { 
                    CollisionBodyType::Circle { radius } => 
                        Some(CircleInstance {
                            position: body.position.into(), 
                            color: body.color.into(), 
                            radius,
                        }),
                    _ => None
                }
        }).collect::<Vec<_>>();

        render_engine.ctx.queue.write_buffer(&render_engine.circle_instance_buffer, 
              0, bytemuck::cast_slice(&instances));
        return instances.len();
    }

    fn write_to_rectangle_instance_buffer(physics_engine: &Box<dyn Simulation>, render_engine: &RenderEngine) -> usize {
        let bodies = physics_engine.get_bodies();
        let instances = bodies.iter().filter_map(
            |body| {
                match body.body_type { 
                    CollisionBodyType::Rectangle{ width, height } => 
                        Some(RectangleInstance {
                            color: body.color.into(), 
                            position: body.position.into(),
                            width,height
                        }),
                    _ => None
                }
            }).collect::<Vec<_>>();

        render_engine.ctx.queue.write_buffer(&render_engine.rectangle_instance_buffer, 
            0, bytemuck::cast_slice(&instances));
        return instances.len();
    }
}

pub struct GameEngineBuilder <'a> {
    physics_engine: Option<Box<dyn Simulation>>,
    render_engine: Option<RenderEngine<'a>>,
}

impl GameEngineBuilder <'static> {
    pub fn new() -> Self {
        Self { physics_engine: None, render_engine: None }
    }

    pub fn physics_engine(mut self, sim: Box<dyn Simulation>) -> Self {
        self.physics_engine = Some(sim);
        self
    }
    
    pub fn render_engine(mut self, engine: RenderEngine<'static>) -> Self {
        self.render_engine = Some(engine);
        self
    }

    pub fn build(self) -> GameEngine<'static>{
        let physics_engine = match self.physics_engine {
            Some(p) => p,
            None => panic!("Physics engine not set."),
        };

        let render_engine = match self.render_engine {
            Some(r) => r,
            None => panic!("Render engine not set."),
        };
       
        GameEngine {
            physics_engine, render_engine
        }
    }
}
