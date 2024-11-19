use crate::engine::event::user_event::UserEvent;
use crate::engine::renderer_engine::asset::background::Background;
use crate::engine::renderer_engine::asset::font::Font;
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteSheet;
use crate::engine::renderer_engine::graphics_context::GraphicsContext;
use crate::engine::renderer_engine::post_process::PostProcessFilterId;
use crate::engine::renderer_engine::render_engine::{
    RenderEngineControl, RenderEngineControlBuilder,
};
use crate::engine::{PhysicsEngine, RenderEngine};
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowId},
};

pub struct GameEngine<'a, T: PhysicsEngine + RenderEngine> {
    window_size: PhysicalSize<u32>,
    window_title: String,
    window: Option<Arc<Window>>,
    last_tick: Instant,
    next_tick: Duration,
    tick_delta: Duration,
    engine: T,
    render_engine_ctl: Option<RenderEngineControl<'a>>,

    // Render engine build info
    sprite_sheet: Option<SpriteSheet>,
    background: Option<Background>,
    font: Option<Font>,
    pp_filter: Vec<PostProcessFilterId>,
}

impl<'a, T: PhysicsEngine + RenderEngine> GameEngine<'a, T> {
    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.run_app(&mut self).expect("Event loop failed");
    }
}

impl<'a, T: PhysicsEngine + RenderEngine> ApplicationHandler for GameEngine<'a, T> {
    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn memory_warning(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("memory_warning");
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Note: The migration to winit 0.30.x resulted in good event handling,
        // but the init of graphics context because kind of messy
        let window_attributes = Window::default_attributes()
            .with_title(&self.window_title)
            .with_inner_size(self.window_size);

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to launch window"),
        );

        // Note: https://github.com/rust-windowing/winit/discussions/3667
        let window_handle = window.clone();
        let g_ctx = GraphicsContext::new(window_handle);
        self.window = Some(window);

        // Build the render engine with data from the physics engine
        let bodies = self.engine.get_bodies();
        let mut render_engine_ctl_builder = RenderEngineControlBuilder::new();
        render_engine_ctl_builder = if let Some(sprite_sheet) = &self.sprite_sheet {
            render_engine_ctl_builder.sprite_sheet(sprite_sheet.clone())
        } else {
            render_engine_ctl_builder
        };

        render_engine_ctl_builder = if let Some(bg) = &self.background {
            render_engine_ctl_builder.background(bg.clone())
        } else {
            render_engine_ctl_builder
        };

        render_engine_ctl_builder = if let Some(f) = &self.font {
            render_engine_ctl_builder.font(f.clone())
        } else {
            render_engine_ctl_builder
        };

        let render_engine_ctl = render_engine_ctl_builder
            .bodies(bodies)
            .add_post_process_filters(&mut self.pp_filter)
            .build(g_ctx, self.window_size);

        self.render_engine_ctl = Some(render_engine_ctl);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    let mut update_count = 0;
                    // Allow at most 5 game updates per frame
                    while self.last_tick.elapsed() > self.next_tick && update_count < 5 {
                        self.engine.update();
                        self.next_tick += self.tick_delta;
                        update_count += 1;
                    }

                    if let Some(ctl) = &mut self.render_engine_ctl {
                        self.engine.render(ctl);
                    }

                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorLeft { .. }
            | WindowEvent::KeyboardInput { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorMoved { .. } => {
                let user_event = UserEvent::from(event);
            }
            _ => (),
        }
    }
}

pub struct GameEngineBuilder<T: PhysicsEngine + RenderEngine> {
    engine: Option<T>,
    sprite_sheet: Option<SpriteSheet>,
    background: Option<Background>,
    window_size: (u32, u32),
    target_fps: u32,
    target_tpf: u32,
    window_title: String,
    font: Option<Font>,
    pp_filter: Vec<PostProcessFilterId>,
}

impl<'a, T: PhysicsEngine + RenderEngine> GameEngineBuilder<T> {
    pub fn new() -> Self {
        let window_size = (800, 600);
        let target_fps = 60;
        let target_tpf = 1;
        Self {
            window_size,
            engine: None,
            sprite_sheet: None,
            target_tpf,
            target_fps,
            background: None,
            window_title: "".to_string(),
            font: None,
            pp_filter: vec![],
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

    pub fn window_size(mut self, window_size: (u32, u32)) -> Self {
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

    pub fn window_title(mut self, title: &str) -> Self {
        self.window_title = title.to_string();
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn add_post_process_filters(mut self, filters: &mut Vec<PostProcessFilterId>) -> Self {
        self.pp_filter.append(filters);
        self
    }

    pub fn build(self) -> GameEngine<'a, T> {
        let (window_width, window_height) = self.window_size;
        let window_size = PhysicalSize::new(window_width, window_height);
        let window_title = self.window_title;
        let window = None; // Initiated by event loop resume fn, by doc recommendation
        let last_tick = Instant::now();
        let tick_delta = Duration::from_millis(1000_u64 / self.target_fps as u64);
        let next_tick = last_tick.elapsed() + tick_delta;
        let engine = self.engine.expect("Physics engine not set");
        let render_engine_ctl = None;
        let sprite_sheet = self.sprite_sheet;
        let background = self.background;
        let font = self.font;
        let pp_filter = self.pp_filter;
        GameEngine {
            window_size,
            window_title,
            window,
            last_tick,
            tick_delta,
            next_tick,
            engine,
            render_engine_ctl,
            sprite_sheet,
            background,
            font,
            pp_filter,
        }
    }
}
