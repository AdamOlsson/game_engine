pub mod naive;
use super::collision::{rigid_body::RigidBody, collision_candidates::CollisionCandidates, CollisionGraph};

pub trait NarrowPhase {
    fn collision_detection(&self, bodies: &mut Vec<RigidBody>,
        candidates: &CollisionCandidates,) -> Option<CollisionGraph>;
}
