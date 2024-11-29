use super::rigid_body::RigidBody;
use super::CollisionInformation;

pub trait CollisionHandler {
    fn handle_circle_circle_collision(
        &self,
        body_i: &mut RigidBody,
        body_j: &mut RigidBody,
    ) -> Option<CollisionInformation>;

    fn handle_circle_rect_collision(
        &self,
        body_i: &mut RigidBody,
        body_j: &mut RigidBody,
    ) -> Option<CollisionInformation>;

    fn handle_rect_rect_collision(
        &self,
        body_i: &mut RigidBody,
        body_j: &mut RigidBody,
    ) -> Option<CollisionInformation>;
}
