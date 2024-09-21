use std::time::{Duration, Instant};

use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy}, keyboard::{KeyCode, PhysicalKey}, window::{WindowBuilder, WindowId}};

use super::{physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType}, renderer_engine::{asset::{background::Background, font::{Font, Writer}}, graphics_context::GraphicsContext, render_engine::{ RenderEngineControl, RenderEngineControlBuilder}, shapes::{circle::CircleInstance, rectangle::RectangleInstance}}, PhysicsEngine, RenderEngine};
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteSheet;

enum CustomEvent {
    ServerTick,
    ClientRender,
}

pub struct GameEngine<'a, T: PhysicsEngine + RenderEngine> {
    physics_engine: T,
    render_engine_ctl: RenderEngineControl<'a>,
    event_loop: EventLoop<CustomEvent>,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    //window_size: PhysicalSize<u32>,
    window_id: WindowId,
    target_fps: u32,
    target_tpf: u32,
}

impl<'a, T: PhysicsEngine + RenderEngine> GameEngine<'a, T> {
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

                        let render_engine_ctl = &mut self.render_engine_ctl; 
                        self.physics_engine.render(render_engine_ctl);
                        
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
}



pub struct GameEngineBuilder<T: PhysicsEngine + RenderEngine> {
    engine: Option<T>,
    sprite_sheet: Option<SpriteSheet>,
    background: Option<Background>,
    window_size: PhysicalSize<u32>,
    target_fps: u32,
    target_tpf: u32,
    window_title: String,
    font: Option<Font>,
}

impl <'a, T: PhysicsEngine + RenderEngine> GameEngineBuilder<T> {
    pub fn new() -> Self {
        let window_size = PhysicalSize::new(800,600);
        let target_fps = 60;
        let target_tpf = 1;
        Self { window_size, engine: None, sprite_sheet: None, target_tpf, target_fps,
            background: None, window_title: "".to_string(), font: None, 
        }
    }

    pub fn engine(mut self, sim: T) -> Self {
        self.engine = Some(sim);
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

    pub fn window_title(mut self, title: String) -> Self {
        self.window_title = title;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn build(self) -> GameEngine<'a, T> {
        let window_size = self.window_size;
        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
            .build()
            .unwrap();
        let window =  WindowBuilder::new()
            .with_title(self.window_title)
            .build(&event_loop).unwrap();
        let window_id = window.id();
        let _ = window.request_inner_size(window_size);
        let event_loop_proxy = event_loop.create_proxy();
        let ctx = GraphicsContext::new(window);
        let physics_engine = self.engine.unwrap();

        // Build the render engine with data from the physics engine
        let bodies = physics_engine.get_bodies();
        let mut render_engine_ctl_builder = RenderEngineControlBuilder::new();
        render_engine_ctl_builder = if let Some(sprite_sheet) = self.sprite_sheet {
            render_engine_ctl_builder.sprite_sheet(sprite_sheet)
        } else { render_engine_ctl_builder };

        render_engine_ctl_builder = if let Some(bg) = self.background {
            render_engine_ctl_builder.background(bg)
        } else  { render_engine_ctl_builder };

        render_engine_ctl_builder = if let Some(f) = self.font {
            render_engine_ctl_builder.font(f)
        } else { render_engine_ctl_builder };

        let render_engine_ctl = render_engine_ctl_builder
            .bodies(bodies)
            .build(ctx, window_size);
       
        let target_fps = self.target_fps;
        let target_tpf = self.target_tpf;
        GameEngine { 
            physics_engine, render_engine_ctl, event_loop, event_loop_proxy, //window_size, 
            window_id, target_tpf, target_fps }
    }
}

