use super::collision::{collision_body::CollisionBody, collision_candidates::CollisionCandidates};

pub mod blockmap;
pub mod sweep_and_prune;
pub mod spatial_subdivision;

pub trait BroadPhase {
    fn collision_detection(&self, bodies: &Vec<CollisionBody>) -> Vec<CollisionCandidates>;
    //fn collision_detection(&self, bodies: &Vec<CollisionBody>) -> Vec<CollisionCandidates>;
}

pub struct BroadPhaseResult<T> {
    collision_candidates: T
}
