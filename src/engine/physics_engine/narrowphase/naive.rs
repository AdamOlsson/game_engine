use crate::engine::physics_engine::collision::{
    collision_candidates::CollisionCandidates,
    collision_handler::CollisionHandler,
    rigid_body::{RigidBody, RigidBodyType},
    CollisionGraph,
};

use super::NarrowPhase;

pub struct Naive<H>
where
    H: CollisionHandler,
{
    solver: H,
}

impl<H> Naive<H>
where
    H: CollisionHandler,
{
    pub fn new(solver: H) -> Self {
        Self { solver }
    }
}

impl<H> NarrowPhase for Naive<H>
where
    H: CollisionHandler,
{
    fn collision_detection(
        &self,
        bodies: &mut Vec<RigidBody>,
        candidates: &CollisionCandidates,
    ) -> Option<CollisionGraph> {
        let num_candidates = candidates.len();

        let mut collisions: Vec<(usize, usize)> = vec![];
        if num_candidates <= 1 {
            return None;
        }

        for i in 0..num_candidates as usize {
            for j in (i + 1)..num_candidates as usize {
                let idx_i = candidates.indices[i];
                let idx_j = candidates.indices[j];
                let body_i = &bodies[idx_i];
                let body_j = &bodies[idx_j];

                let (type_i, type_j) = (&body_i.body_type, &body_j.body_type);
                let collision_info = match (type_i, type_j) {
                    (RigidBodyType::Circle { .. }, RigidBodyType::Circle { .. }) => self
                        .solver
                        .handle_circle_circle_collision(bodies, idx_i, idx_j),

                    (RigidBodyType::Rectangle { .. }, RigidBodyType::Rectangle { .. }) => {
                        self.solver.handle_rect_rect_collision(bodies, idx_i, idx_j)
                    }

                    (RigidBodyType::Rectangle { .. }, RigidBodyType::Circle { .. }) => self
                        .solver
                        .handle_circle_rect_collision(bodies, idx_j, idx_i),

                    (RigidBodyType::Circle { .. }, RigidBodyType::Rectangle { .. }) => self
                        .solver
                        .handle_circle_rect_collision(bodies, idx_i, idx_j),

                    (_, _) => panic!("Unkown body type collision {type_i} and {type_j}"),
                };

                if let Some(_info) = collision_info {
                    collisions.push((idx_i, idx_j));
                }
            }
        }
        match collisions.len() {
            0 => None,
            _ => Some(CollisionGraph { collisions }),
        }
    }
}
