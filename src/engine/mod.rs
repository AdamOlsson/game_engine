pub mod init_utils;
pub mod physics_engine;
pub mod renderer_engine;
pub mod game_engine;
pub mod util;

use physics_engine::collision::collision_body::CollisionBody;

#[allow(dead_code)]
pub trait State {
    fn get_bodies(&self) -> &Vec<CollisionBody>;
    fn get_bodies_mut(&mut self) -> &mut Vec<CollisionBody>;
}

pub trait Simulation {
    fn update(&mut self);
    fn get_bodies(&self) -> &Vec<CollisionBody>;

    // Interactions
    fn jump(&mut self) {}
}

pub trait Interaction {
}
