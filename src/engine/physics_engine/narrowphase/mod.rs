pub mod naive;
use super::collision::{
    collision_candidates::CollisionCandidates, rigid_body::RigidBody, CollisionGraph,
};

pub trait NarrowPhase {
    fn collision_detection(
        &self,
        bodies: &mut Vec<&mut RigidBody>,
        candidates: &CollisionCandidates,
    ) -> Option<CollisionGraph>;
}
