use crate::engine::physics_engine::collision::{
    collision_candidates::CollisionCandidates,
    collision_handler::CollisionHandler,
    rigid_body::{RigidBody, RigidBodyType},
    CollisionGraph, CollisionGraphNode,
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

        let mut collisions: Vec<CollisionGraphNode> = vec![];
        if num_candidates <= 1 {
            return None;
        }

        for i in 0..num_candidates as usize {
            for j in (i + 1)..num_candidates as usize {
                let idx_i = candidates.indices[i];
                let idx_j = candidates.indices[j];

                let (min_idx, max_idx) = if idx_i < idx_j {
                    (idx_i, idx_j)
                } else {
                    (idx_j, idx_i)
                };
                let (left, right) = bodies.split_at_mut(max_idx);

                let mut body_i = &mut left[min_idx];
                let mut body_j = &mut right[0];

                let collision_info = match (&body_i.body_type, &body_j.body_type) {
                    (RigidBodyType::Circle { .. }, RigidBodyType::Circle { .. }) => self
                        .solver
                        .handle_circle_circle_collision(&mut body_i, &mut body_j),
                    (RigidBodyType::Rectangle { .. }, RigidBodyType::Rectangle { .. }) => self
                        .solver
                        .handle_rect_rect_collision(&mut body_i, &mut body_j),

                    (RigidBodyType::Rectangle { .. }, RigidBodyType::Circle { .. }) => self
                        .solver
                        .handle_circle_rect_collision(&mut body_j, &mut body_i),

                    (RigidBodyType::Circle { .. }, RigidBodyType::Rectangle { .. }) => self
                        .solver
                        .handle_circle_rect_collision(&mut body_i, &mut body_j),

                    (_, _) => panic!("Unkown body type collision {body_i} and {body_j}"),
                };

                if let Some(info) = collision_info {
                    collisions.push(CollisionGraphNode {
                        body_i_idx: idx_i,
                        body_j_idx: idx_j,
                        info,
                    });
                }
            }
        }
        match collisions.len() {
            0 => None,
            _ => Some(CollisionGraph { collisions }),
        }
    }
}
