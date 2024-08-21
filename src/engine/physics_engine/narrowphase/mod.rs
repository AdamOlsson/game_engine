pub mod naive;
use super::collision::{collision_body::CollisionBody, collision_candidates::CollisionCandidates, CollisionGraph};

pub trait NarrowPhase {
    fn collision_detection(&self, bodies: &mut Vec<CollisionBody>,
        candidates: &CollisionCandidates,) -> CollisionGraph;
}
