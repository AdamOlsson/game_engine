
use crate::engine::{game_engine, Simulation};
use winit::window::WindowBuilder;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::event_loop::EventLoopBuilder;
use winit::event::*;
use winit::dpi::PhysicalSize;
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}

pub async fn run<S: Simulation + 'static>(simulation: S, window_size: PhysicalSize<u32>, update_freq_ms: u32) {
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();
    let window =  WindowBuilder::new().build(&event_loop).unwrap();
    let _ = window.request_inner_size(window_size);

    let mut game_engine = game_engine::GameEngineBuilder::new()
        .physics_engine(Box::new(simulation))
        .build(window).await;
    
    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(update_freq_ms as u64));
        event_loop_proxy.send_event(CustomEvent::Timer).ok();
    });

    event_loop.run(
        move | event, elwt | match event {
            Event::UserEvent(..) => {
                let _before = Instant::now();
                game_engine.update();
                game_engine.tick().unwrap();
                let _after = Instant::now();
                //println!("{:?}", _after.duration_since(_before));
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == game_engine.render_engine.ctx.window_id => match event {
                WindowEvent::Resized(physical_size) => game_engine.resize(*physical_size),

                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
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

                WindowEvent::RedrawRequested => {
                    game_engine.tick().unwrap();
                } 

                WindowEvent::KeyboardInput { event: 
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Space),
                        repeat: false,
                        ..
                    },
                    ..
                } => game_engine.send_keyboard_input(KeyCode::Space),
                _ => (),
            },
            _ => ()
        }
    ).expect("Error!");
}

