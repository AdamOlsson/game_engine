pub mod rigid_body;
pub mod collision_candidates;
pub mod collision_handler;


pub struct CollisionGraph {
    pub collisions: Vec<(usize, usize)>,
}

