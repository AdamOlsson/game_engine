pub mod entity;
pub mod event;
pub mod game_engine;
pub mod init_utils;
pub mod physics_engine;
pub mod renderer_engine;
pub mod util;

use event::user_event::UserEvent;
use physics_engine::collision::RigidBody;
use renderer_engine::RenderEngineControl;

#[allow(unused_variables)]
pub trait PhysicsEngine {
    fn update(&mut self);
    fn get_bodies(&self) -> Vec<&RigidBody>;

    fn user_event(&mut self, event: UserEvent) {}
}

pub trait RenderEngine {
    fn render(&mut self, engine_ctl: &mut RenderEngineControl);
}
