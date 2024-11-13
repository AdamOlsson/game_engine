pub mod sat;

#[derive(Debug)]
pub struct SATCollisionInfo {
    pub penetration_depth: f32,
    pub normal: [f32; 3],
    pub collision_point: [f32; 3],
}
