use crate::engine::physics_engine::collision::RigidBody;

pub mod elastic;
pub mod inelastic;
pub mod none;

#[allow(unused_variables)]
pub trait ConstraintResolver {
    fn resolve_vertical(&self, diff: f32, body: &mut RigidBody) {}
    fn resolve_horizontal(&self, diff: f32, body: &mut RigidBody) {}
}
