use super::collision::collision_body::CollisionBody;

pub mod blockmap;
pub mod spatial_subdivision;

pub trait BroadPhase<T> {
    fn collision_detection(&self, bodies: &Vec<CollisionBody>) -> T;
}

