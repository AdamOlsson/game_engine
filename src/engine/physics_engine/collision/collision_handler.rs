use super::collision_body::{CollisionBody, CollisionBodyType};
use cgmath::InnerSpace;


pub trait CollisionHandler {
    fn handle_collision(&self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize);
}

pub struct SimpleCollisionSolver {}
impl SimpleCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for SimpleCollisionSolver {
    fn handle_collision(
        &self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize) 
    {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];
        let (radius_i, radius_j) = match (&body_i.body_type, &body_j.body_type) {
            (CollisionBodyType::Circle { radius: ri }, CollisionBodyType::Circle { radius: rj}) =>
                (ri, rj),
            (_, _) => panic!()
        };
        let collision_axis = body_i.position - body_j.position;
        let collision_normal = collision_axis.normalize();
        let dist = collision_axis.magnitude();
        let correction_direction = collision_axis / dist;
        let collision_depth = radius_i + radius_j- dist;

        bodies[idx_i].position += 0.5*collision_depth*correction_direction;
        bodies[idx_j].position -= 0.5*collision_depth*correction_direction;

        let p = bodies[idx_i].velocity.dot(collision_normal) - bodies[idx_j].velocity.dot(collision_normal);
        bodies[idx_i].velocity = bodies[idx_i].velocity - p * collision_normal; 
        bodies[idx_j].velocity = bodies[idx_j].velocity + p * collision_normal;

        bodies[idx_i].prev_position = bodies[idx_i].position - bodies[idx_i].velocity;
        bodies[idx_j].prev_position = bodies[idx_j].position - bodies[idx_j].velocity;
    }
}
