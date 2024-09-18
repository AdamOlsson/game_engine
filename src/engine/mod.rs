pub mod init_utils;
pub mod physics_engine;
pub mod renderer_engine;
pub mod game_engine;
pub mod util;

use physics_engine::collision::collision_body::CollisionBody;

pub trait PhysicsEngine {
    fn update(&mut self);
    fn get_bodies(&self) -> &Vec<CollisionBody>;

    // Interactions
    fn jump(&mut self) {}
}

