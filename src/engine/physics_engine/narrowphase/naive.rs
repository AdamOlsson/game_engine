use cgmath::{MetricSpace, Vector3};
use crate::engine::physics_engine::collision::{collision_body::{CollisionBody, CollisionBodyType}, collision_candidates::CollisionCandidates, collision_handler::CollisionHandler, CollisionGraph};

use super::NarrowPhase;

pub struct Naive<H>
    where H: CollisionHandler
{
    solver: H,
}

impl <H> Naive <H>
    where 
        H: CollisionHandler,
{
    pub fn new(solver: H) -> Self {
        Self { solver }
    }

    fn circle_rectangle_test_for_collision(
        circ_pos: &Vector3<f32>, circ_radius: f32, rect_pos: &Vector3<f32>,
        rect_width: f32, rect_height: f32
    ) -> bool {
        
        let test_edge_x = 
            if circ_pos.x < rect_pos.x {
                rect_pos.x
            } else if circ_pos.x > rect_pos.x + rect_width {
                rect_pos.x + rect_width
            } else {
                circ_pos.x
            };
        let test_edge_y = 
            if circ_pos.y < rect_pos.y {
                rect_pos.y
            } else if circ_pos.y > rect_pos.y + rect_height {
                rect_pos.y + rect_height
            } else {
                circ_pos.y
            };
        let dist = ((circ_pos.x - test_edge_x).powi(2) + (circ_pos.y - test_edge_y).powi(2)).sqrt();
        return dist <= circ_radius;
    }
}

impl <H> NarrowPhase for Naive<H>
    where
        H: CollisionHandler,
{
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

                    (CollisionBodyType::Rectangle { width, height },
                     CollisionBodyType::Circle { radius }) => 
                        if Self::circle_rectangle_test_for_collision(
                                &body_j.position, *radius, &body_i.position, *width, *height) {
                            collisions.push((idx_i, idx_j));
                        },

                    (CollisionBodyType::Circle { radius },
                     CollisionBodyType::Rectangle { width, height }) => 
                        if Self::circle_rectangle_test_for_collision(
                                &body_i.position, *radius, &body_j.position, *width, *height) {
                            collisions.push((idx_i, idx_j));
                        },
                    //(_, _) => panic!(),
                }
            }
        } 
        return CollisionGraph { collisions }
    }
}
