use std::time::{Duration, Instant};

use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy}, keyboard::{KeyCode, PhysicalKey}, window::{WindowBuilder, WindowId}};

use super::{physics_engine::collision::collision_body::CollisionBodyType, renderer_engine::{asset::{background::Background, font::{Font, Writer}}, graphics_context::GraphicsContext, render_engine::{self, RenderEngine, RenderEngineBuilder}, shapes::{circle::CircleInstance, rectangle::RectangleInstance}}, Simulation};
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteSheet;

enum CustomEvent {
    ServerTick,
    ClientRender,
}

pub struct GameEngine<'a> {
    physics_engine: Box<dyn Simulation + 'static>,
    render_engine: RenderEngine<'a>,
    event_loop: EventLoop<CustomEvent>,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    window_size: PhysicalSize<u32>,
    window_id: WindowId,
    target_fps: u32,
    target_tpf: u32,
    writer: Writer,
}

impl<'a> GameEngine<'a> {
    pub fn run(mut self) {

        let mut tick_count = 0;
        let hz = Duration::from_millis((1000/self.target_fps) as u64);
        let mut time_since_render = Instant::now();
        std::thread::spawn(move || loop {
            if tick_count < self.target_tpf {
                self.event_loop_proxy.send_event(CustomEvent::ServerTick).ok();
                tick_count += 1;
                continue;
            } 

            if time_since_render.elapsed() > hz {
                self.event_loop_proxy.send_event(CustomEvent::ClientRender).ok();
                tick_count = 0;
                time_since_render = Instant::now();
                continue;
            } 

        });
        
        let mut num_ticks = 0;
        let mut num_renders = 0;
        let mut total_tick_time = Duration::from_millis(0);
        let mut total_render_time = Duration::from_millis(0);
        let statistics_interval = Duration::from_secs(5);
        let mut statistics_timer_last_print = Instant::now();
        self.event_loop.run(move | event, elwt | match event {
            Event::UserEvent(e) => { 
                match e {
                    CustomEvent::ServerTick => {
                        let now = Instant::now();
                        self.physics_engine.update();
                        //std::thread::sleep(Duration::from_millis(300));
                        let time = now.elapsed();
                        total_tick_time += time;
                        num_ticks += 1;
                    },
                    CustomEvent::ClientRender => {
                        let now = Instant::now();
                        let rect_instances = Self::get_rectangle_instances(&self.physics_engine);
                        let circle_instances = Self::get_circle_instances(&self.physics_engine);
                        let _ = self.render_engine.render_background();
                        let _ = self.render_engine.render_rectangles(&rect_instances, false);
                        let _ = self.render_engine.render_circles(&circle_instances, false);

                        let text = self.writer.write("HELLOWORLD");
                        let _ = self.render_engine.render_text(text, false);

                        let _ = self.render_engine.post_process();

                        total_render_time += now.elapsed();
                        num_renders += 1;

                        if now.duration_since(statistics_timer_last_print) > statistics_interval {
                            let avg_fps = 1000 / (total_render_time.as_millis().max(1) / num_renders).max(1);
                            let avg_tps = 1000 / (total_tick_time.as_millis().max(1) / num_ticks).max(1);
                            println!("{}FPS, {}TPS", avg_fps, avg_tps);
                            statistics_timer_last_print = now;
                            
                            num_ticks = 0;
                            num_renders = 0;
                            total_tick_time = Duration::from_millis(0);
                            total_render_time = Duration::from_millis(0);
                        }
                        
                    },
                }
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
                            sprite_coord: body.sprite_coord.coordinate, 
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
    sprite_sheet: Option<SpriteSheet>,
    background: Option<Background>,
    window_size: PhysicalSize<u32>,
    target_fps: u32,
    target_tpf: u32,
}

impl <'a> GameEngineBuilder {
    pub fn new() -> Self {
        let window_size = PhysicalSize::new(800,600);
        let target_fps = 60;
        let target_tpf = 1;
        Self { window_size, physics_engine: None, sprite_sheet: None, target_tpf, target_fps,
            background: None,
        }
    }

    pub fn physics_engine<S: Simulation + 'static>(mut self, sim: S) -> Self {
        self.physics_engine = Some(Box::new(sim));
        self
    }

    pub fn sprite_sheet(mut self, tex: SpriteSheet) -> Self {
        self.sprite_sheet = Some(tex);
        self
    }

    pub fn background(mut self, background: Background) -> Self {
        self.background = Some(background);
        self
    }

    pub fn window_size(mut self, window_size: PhysicalSize<u32>) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn target_ticks_per_frame(mut self, n: u32) -> Self {
        self.target_tpf = n;
        self
    }

    pub fn target_frames_per_sec(mut self, n: u32) -> Self {
        self.target_fps = n;
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
        let mut render_engine_builder = RenderEngineBuilder::new();
        render_engine_builder = if let Some(sprite_sheet) = self.sprite_sheet {
            render_engine_builder.sprite_sheet(sprite_sheet)
        } else  {
            render_engine_builder
        };

        render_engine_builder = if let Some(bg) = self.background {
            render_engine_builder.background(bg)
        } else  {
            render_engine_builder
        };

        let font = Font::new(include_bytes!("./renderer_engine/asset/fonts/font.png"), 11, 11);
        let writer = font.writer();
        let render_engine = render_engine_builder.bodies(bodies)
            .font(font)
            .build(ctx, window_size);
       
        let target_fps = self.target_fps;
        let target_tpf = self.target_tpf;
        GameEngine { 
            physics_engine, render_engine, event_loop, event_loop_proxy, window_size, 
            window_id, target_tpf, target_fps, writer }
    }
}

