pub mod naive;
use super::collision::{collision_candidates::CollisionCandidates, CollisionGraph, RigidBody};

pub trait NarrowPhase {
    fn collision_detection(
        &self,
        bodies: &mut Vec<&mut RigidBody>,
        candidates: &CollisionCandidates,
    ) -> Option<CollisionGraph>;
}
