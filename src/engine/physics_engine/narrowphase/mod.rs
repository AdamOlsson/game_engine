use super::collision::{collision_body::CollisionBody, collision_candidates::CollisionCandidates};



pub mod naive;
pub trait NarrowPhase {
    fn collision_detection(
        &self, 
        bodies: &mut Vec<CollisionBody>,
        candidates: &CollisionCandidates, 
    );
}
