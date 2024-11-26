use super::collision::RigidBody;

mod blockmap;
mod spatial_subdivision;

pub use blockmap::BlockMap;
pub use spatial_subdivision::spatial_subdivision::SpatialSubdivision;

pub trait BroadPhase<T> {
    fn collision_detection<'a, I>(&self, bodies: I) -> T
    where
        I: Iterator<Item = &'a RigidBody>;
}
