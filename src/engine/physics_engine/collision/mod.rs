pub mod rigid_body;
pub mod collision_candidates;
pub mod collision_handler;


#[derive(Debug)]
pub struct CollisionGraph {
    pub collisions: Vec<(usize, usize)>,
}

impl std::fmt::Display for CollisionGraph{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self.collisions);
        write!(f, "CollisionGraph{{ {s} }}")
    }
}
