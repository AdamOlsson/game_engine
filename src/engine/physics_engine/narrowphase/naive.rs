use cgmath::{MetricSpace, Vector3};
use crate::engine::physics_engine::collision::{rigid_body::{RigidBody, RigidBodyType}, collision_candidates::CollisionCandidates, collision_handler::CollisionHandler, CollisionGraph};

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
        let temp_circle_pos = circ_pos - rect_pos;
        let closest_point_x = (-rect_width/2.0).max((rect_width/2.0).min(temp_circle_pos.x));
        let closest_point_y = (-rect_height/2.0).max((rect_height/2.0).min(temp_circle_pos.y));
        let closes_point = Vector3::new(closest_point_x, closest_point_y, 0.0);
        return closes_point.distance2(temp_circle_pos) <= circ_radius.powi(2);
    }
}

impl <H> NarrowPhase for Naive<H>
    where
        H: CollisionHandler,
{
    fn collision_detection(&self, bodies: &mut Vec<RigidBody>,
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
                debug_assert!(dist != 0.0, "Collision axis has zero length.");
                
                let (type_i, type_j) = (&body_i.body_type, &body_j.body_type);
                match (type_i, type_j) {
                    (RigidBodyType::Circle { radius: ri }, RigidBodyType::Circle { radius: rj}) =>
                        if dist < (ri + rj) {
                            self.solver.handle_circle_circle_collision(bodies, idx_i, idx_j);
                            collisions.push((idx_i, idx_j));
                        },

                    (RigidBodyType::Rectangle { width: wi, height:hi },
                     RigidBodyType::Rectangle { width: wj, height:hj }) => 
                        if body_i.position.x + wi/2.0 >= body_j.position.x &&
                                body_i.position.x <= body_j.position.x + wj/2.0 &&
                                body_i.position.y + hi/2.0 >= body_j.position.y &&
                                body_i.position.y <= body_j.position.y + hj/2.0 {
                            self.solver.handle_rect_rect_collision(bodies, idx_i, idx_j);
                            collisions.push((idx_i, idx_j));
                        },

                    (RigidBodyType::Rectangle { width, height },
                     RigidBodyType::Circle { radius }) => 
                        if Self::circle_rectangle_test_for_collision(
                                &body_j.position, *radius, &body_i.position, *width, *height) {
                            self.solver.handle_circle_rect_collision(bodies, idx_j, idx_i);
                            collisions.push((idx_i, idx_j));
                        },

                    (RigidBodyType::Circle { radius },
                     RigidBodyType::Rectangle { width, height }) => 
                        if Self::circle_rectangle_test_for_collision(
                                &body_i.position, *radius, &body_j.position, *width, *height) {
                            self.solver.handle_circle_rect_collision(bodies, idx_i, idx_j);
                            collisions.push((idx_i, idx_j));
                        },
                    //(_, _) => panic!(),
                }
            }
        } 
        return CollisionGraph { collisions }
    }
}
