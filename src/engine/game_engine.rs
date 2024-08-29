use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy}, keyboard::{KeyCode, PhysicalKey}, window::{WindowBuilder, WindowId}};

use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::{graphics_context::GraphicsContext, render_engine::{self, RenderEngine}, shapes::{circle::CircleInstance, rectangle::RectangleInstance}}, Simulation};

enum CustomEvent {
    Timer,
}

pub struct GameEngine<'a> {
    physics_engine: Box<dyn Simulation + 'static>,
    render_engine: RenderEngine<'a>,
    event_loop: EventLoop<CustomEvent>,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    window_size: PhysicalSize<u32>,
    window_id: WindowId,
}

impl<'a> GameEngine<'a> {
    pub fn run(mut self) {

        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(0));
            self.event_loop_proxy.send_event(CustomEvent::Timer).ok();
        });
       
        self.event_loop.run(move | event, elwt | match event {
            Event::UserEvent(..) => {
                self.physics_engine.update(); 
                let num_rect_instances = Self::write_to_rectangle_instance_buffer(&self.physics_engine, &self.render_engine);
                let num_circle_instances = Self::write_to_circle_instance_buffer(&self.physics_engine, &self.render_engine);
                let _ = self.render_engine.render_rectangles(num_rect_instances as u32, true);
                let _ = self.render_engine.render_circles(num_circle_instances as u32, false);

                let _ = self.render_engine.post_process();
            },

            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == self.window_id => match event {
                //WindowEvent::Resized(physical_size) => game_engine.resize(*physical_size),

                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent{
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => {
                    println!("Goodbye, see you!");
                    elwt.exit();
                }

                //WindowEvent::RedrawRequested => {
                //    game_engine.tick().unwrap();
                //} 

                WindowEvent::KeyboardInput { event: 
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Space),
                        repeat: false,
                        ..
                    },
                    ..
                } => self.physics_engine.jump(), 
                _ => (),
            },
            _ => (),
        }).unwrap();
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
                        Some(RectangleInstance{
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


pub struct GameEngineBuilder {
    physics_engine: Option<Box<dyn Simulation>>,
    window_size: PhysicalSize<u32>
}
impl <'a> GameEngineBuilder {
    pub fn new() -> Self {
        let window_size = PhysicalSize::new(800,600);
        Self { window_size, physics_engine: None }
    }

    pub fn physics_engine<S: Simulation + 'static>(mut self, sim: S) -> Self {
        self.physics_engine = Some(Box::new(sim));
        self
    }

    pub fn window_size(mut self, window_size: PhysicalSize<u32>) -> Self {
        self.window_size = window_size;
        self
    }


    pub fn build(self) -> GameEngine<'a> {
        let window_size = self.window_size;
        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
            .build()
            .unwrap();
        let window =  WindowBuilder::new().build(&event_loop).unwrap();
        let window_id = window.id();
        let _ = window.request_inner_size(window_size);
        let event_loop_proxy = event_loop.create_proxy();
        let ctx = GraphicsContext::new(window);
        let physics_engine = self.physics_engine.unwrap();

        // Build the render engine with data from the physics engine
        let bodies = physics_engine.get_bodies();
        let render_engine = render_engine::RenderEngineBuilder::new()
            .bodies(&bodies)
            .build(ctx,self.window_size);

        GameEngine { 
            physics_engine, render_engine, event_loop, event_loop_proxy, window_size, window_id }
    }
}

