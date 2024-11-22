pub mod collision_candidates;
pub mod collision_handler;
pub mod identity_collision_handler;
pub mod rigid_body;
pub mod sat;

#[derive(Debug)]
pub struct CollisionGraph {
    pub collisions: Vec<(usize, usize)>,
}

impl std::fmt::Display for CollisionGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self.collisions);
        write!(f, "CollisionGraph{{ {s} }}")
    }
}

#[derive(Debug)]
pub struct CollisionInformation {
    pub penetration_depth: f32,
    pub normal: [f32; 3],
    pub collision_point: [f32; 3],
}
