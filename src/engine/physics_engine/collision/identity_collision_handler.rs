use super::collision_handler::CollisionHandler;
use super::rigid_body::RigidBody;
use super::CollisionInformation;

pub struct IdentityCollisionSolver {}

impl IdentityCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for IdentityCollisionSolver {
    fn handle_circle_circle_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> Option<CollisionInformation> {
        None
    }
    fn handle_circle_rect_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> Option<CollisionInformation> {
        None
    }
    fn handle_rect_rect_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> Option<CollisionInformation> {
        None
    }
}
