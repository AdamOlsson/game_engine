use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy}, keyboard::{KeyCode, PhysicalKey}, window::{WindowBuilder, WindowId}};

use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::{graphics_context::GraphicsContext, render_engine::{self, RenderEngine}, shapes::{circle::CircleInstance, rectangle::RectangleInstance}, sprite_sheet::SpriteSheet}, Simulation};

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
            let fps = (1000.0 / 60.0) as u64;
            std::thread::sleep(std::time::Duration::from_millis(fps));
            self.event_loop_proxy.send_event(CustomEvent::Timer).ok();
        });
       
        self.event_loop.run(move | event, elwt | match event {
            Event::UserEvent(..) => {
                self.physics_engine.update(); 
                let rect_instances = Self::get_rectangle_instances(&self.physics_engine);
                let circle_instances = Self::get_circle_instances(&self.physics_engine);
                let _ = self.render_engine.render_rectangles(&rect_instances, true);
                let _ = self.render_engine.render_circles(&circle_instances, false);

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

    fn get_circle_instances(physics_engine: &Box<dyn Simulation>) -> Vec<CircleInstance> {
        let bodies = physics_engine.get_bodies();
        bodies.iter().filter_map(
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
        }).collect::<Vec<_>>()
    }

    fn get_rectangle_instances(physics_engine: &Box<dyn Simulation>) -> Vec<RectangleInstance> {
        let bodies = physics_engine.get_bodies();
        bodies.iter().filter_map(
            |body| {
                match body.body_type { 
                    CollisionBodyType::Rectangle{ width, height } => 
                        Some(RectangleInstance{
                            color: body.color.into(), 
                            position: body.position.into(),
                            width,height,
                            sprite_coord: body.sprite_coord.coordinate,
                        }),
                    _ => None
                }
            }).collect::<Vec<_>>()
    }
}


pub struct GameEngineBuilder {
    physics_engine: Option<Box<dyn Simulation>>,
    texture: Option<SpriteSheet>,
    window_size: PhysicalSize<u32>
}
impl <'a> GameEngineBuilder {
    pub fn new() -> Self {
        let window_size = PhysicalSize::new(800,600);
        Self { window_size, physics_engine: None, texture: None }
    }

    pub fn physics_engine<S: Simulation + 'static>(mut self, sim: S) -> Self {
        self.physics_engine = Some(Box::new(sim));
        self
    }

    pub fn texture(mut self, tex: SpriteSheet) -> Self {
        self.texture = Some(tex);
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
        let render_engine = if let Some(tex) = self.texture {
            render_engine::RenderEngineBuilder::new()
                .bodies(&bodies)
                .texture(tex)
                .build(ctx,self.window_size)
        } else  {
            render_engine::RenderEngineBuilder::new()
                .bodies(&bodies)
                .build(ctx,self.window_size)
        };
        
        GameEngine { 
            physics_engine, render_engine, event_loop, event_loop_proxy, window_size, window_id }
    }
}

