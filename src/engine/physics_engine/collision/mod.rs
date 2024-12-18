pub mod collision_candidates;
pub mod collision_handler;
pub mod identity_collision_handler;
mod rigid_body;
pub mod sat;
mod simple_collision_handler;

pub use collision_handler::CollisionHandler;
pub use rigid_body::{RigidBody, RigidBodyBuilder, RigidBodyType};
pub use simple_collision_handler::SimpleCollisionSolver;

#[derive(Debug)]
pub struct CollisionGraph {
    pub collisions: Vec<CollisionGraphNode>,
}

#[derive(Debug)]
pub struct CollisionGraphNode {
    pub body_i_idx: usize,
    pub body_j_idx: usize,
    pub info: CollisionInformation,
}

impl std::fmt::Display for CollisionGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self.collisions);
        write!(f, "CollisionGraph{{ {s} }}")
    }
}

#[derive(Debug)]
pub struct CollisionInformation {
    pub penetration_depth: f32,
    pub normal: [f32; 3],
    pub collision_point: [f32; 3],
}
