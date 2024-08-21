use cgmath::MetricSpace;
use crate::engine::physics_engine::collision::{collision_body::{CollisionBody, CollisionBodyType}, collision_candidates::CollisionCandidates, collision_handler::CollisionHandler, CollisionGraph};

use super::NarrowPhase;

pub struct Naive {
    solver: Box<dyn CollisionHandler + 'static>
}

impl Naive {
    pub fn new<H>(solver: H) -> Self
    where
        H: CollisionHandler + 'static,
    {
        let s: Box<dyn CollisionHandler> = Box::new(solver);
        Self { solver: s }
    }
}

impl NarrowPhase for Naive {
    fn collision_detection(&self, bodies: &mut Vec<CollisionBody>,
        candidates: &CollisionCandidates,
    ) -> CollisionGraph {
        let num_candidates = candidates.len();

        let mut collisions: Vec<(usize,usize)> = vec![];
        if num_candidates <= 1 {
            return CollisionGraph{ collisions };
        }

        for i in 0..num_candidates as usize {
            for j in 0..num_candidates as usize {
                if i == j {
                    continue;
                }
                let idx_i = candidates.indices[i];
                let idx_j = candidates.indices[j];
                let body_i = &bodies[idx_i];
                let body_j = &bodies[idx_j];

                let dist = body_i.position.distance(body_j.position);
                if dist == 0.0 {
                    panic!("Collision axis has zero length.");
                }
                let (type_i, type_j) = (&body_i.body_type, &body_j.body_type);
                match (type_i, type_j) {
                    (CollisionBodyType::Circle { radius: ri }, CollisionBodyType::Circle { radius: rj}) =>
                        if dist < (ri + rj) {
                            self.solver.handle_circle_circle_collision(bodies, idx_i, idx_j);
                            collisions.push((idx_i, idx_j));
                        },
                    (CollisionBodyType::Rectangle { width: wi, height:hi },
                     CollisionBodyType::Rectangle { width: wj, height:hj }) => 
                        if body_i.position.x + wi >= body_j.position.x &&
                                body_i.position.x <= body_j.position.x + wj &&
                                body_i.position.y + hi >= body_j.position.y &&
                                body_i.position.y <= body_j.position.y + hj {
                            self.solver.handle_rect_rect_collision(bodies, idx_i, idx_j);
                            collisions.push((idx_i, idx_j));
                        },
                    (CollisionBodyType::Rectangle { width: _, height: _ },
                     CollisionBodyType::Circle { radius: _ }) |
                    (CollisionBodyType::Circle { radius: _ },
                     CollisionBodyType::Rectangle { width: _, height: _}) => (),
                    //(_, _) => panic!(),
                }
            }
        } 

        return CollisionGraph { collisions }
    }
}
