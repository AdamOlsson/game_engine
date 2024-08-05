use super::collision::{collision_body::CollisionBody, collision_candidates::CollisionCandidates};


pub mod blockmap;
pub mod sweep_and_prune;

pub trait BroadPhase {
    fn collision_detection(&self, bodies: &Vec<CollisionBody>) -> Vec<CollisionCandidates>;
}
