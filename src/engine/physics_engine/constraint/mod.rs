use super::collision::collision_body::CollisionBody;

pub mod circle_constraint;
pub mod box_constraint;
pub mod resolver;

pub trait Constraint {
    fn apply_constraint(&self, body: &mut CollisionBody);
}
