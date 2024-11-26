use super::collision::rigid_body::RigidBody;

pub mod blockmap;
pub mod spatial_subdivision;

pub trait BroadPhase<T> {
    fn collision_detection<'a, I>(&self, bodies: I) -> T
    where
        I: Iterator<Item = &'a RigidBody>;
}
