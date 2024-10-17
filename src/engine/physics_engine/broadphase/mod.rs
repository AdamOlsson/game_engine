use super::collision::rigid_body::RigidBody;

pub mod blockmap;
pub mod spatial_subdivision;

pub trait BroadPhase<T> {
    fn collision_detection(&self, bodies: &Vec<RigidBody>) -> T;
}

