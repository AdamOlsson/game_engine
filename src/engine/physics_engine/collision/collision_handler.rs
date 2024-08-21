use super::collision_body::{CollisionBody, CollisionBodyType};
use cgmath::{InnerSpace};


pub trait CollisionHandler {
    fn handle_circle_circle_collision(&self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize);
    fn handle_circle_rect_collision(&self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize);
    fn handle_rect_rect_collision(&self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize);
}

pub struct SimpleCollisionSolver {}
impl SimpleCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for SimpleCollisionSolver {
    fn handle_circle_circle_collision(
        &self, bodies: &mut Vec<CollisionBody>, idx_i: usize, idx_j: usize
    ) {
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
        let collision_depth = radius_i + radius_j - dist;

        bodies[idx_i].position += 0.5*collision_depth*correction_direction;
        bodies[idx_j].position -= 0.5*collision_depth*correction_direction;

        let p = bodies[idx_i].velocity.dot(collision_normal) - bodies[idx_j].velocity.dot(collision_normal);
        bodies[idx_i].velocity = bodies[idx_i].velocity - p * collision_normal; 
        bodies[idx_j].velocity = bodies[idx_j].velocity + p * collision_normal;

        bodies[idx_i].prev_position = bodies[idx_i].position - bodies[idx_i].velocity;
        bodies[idx_j].prev_position = bodies[idx_j].position - bodies[idx_j].velocity;
    }

    fn handle_circle_rect_collision(
        &self, _bodies: &mut Vec<CollisionBody>, _idx_i: usize, _idx_j: usize
    ) {}

    fn handle_rect_rect_collision(
        &self, _bodies: &mut Vec<CollisionBody>, _idx_i: usize, _idx_j: usize
    ) {
        //let body_i = &bodies[idx_i];
        //let body_j = &bodies[idx_j];
        //let ((wi,hi), (wj, hj)) = match (&body_i.body_type, &body_j.body_type) {
        //    (CollisionBodyType::Rectangle { width: wi, height:hi },
        //     CollisionBodyType::Rectangle { width: wj, height:hj }) => ((wi, hi), (wj,hj)), 
        //    (_, _) => panic!()
        //};   
        //let collision_axis = body_i.position - body_j.position;
        //let collision_normal = collision_axis.normalize();
        //let dist = collision_axis.magnitude();
        //let correction_direction = collision_axis / dist;
        //let collision_depth_w = wi + wj - (body_j.position.x + wj - body_i.position.x);
        ////let collision_depth_w = f32::min(body_i.position.x + wi, body_j.position.x + wj) -
        ////    f32::max(body_i.position.x, body_j.position.x);

        ////let collision_depth_h = hi + hj - (body_j.position.y + hj - body_i.position.y);
        //let collision_depth_h = f32::min(body_i.position.y + hi, body_j.position.y + hj) -
        //    f32::max(body_i.position.y, body_j.position.y);

        ////let collision_depth = Vector3::new(collision_depth_w, collision_depth_h, 0.0).magnitude();
        //let collision_depth = collision_depth_w.min(collision_depth_h);

        //println!("collision depth w: {:?}", collision_depth_w);
        //println!("collision depth h: {:?}", collision_depth_h);
        //println!("collision depth magnitude: {:?}", collision_depth);
        //bodies[idx_i].position += 0.5*collision_depth*correction_direction;
        //bodies[idx_j].position -= 0.5*collision_depth*correction_direction;
        //
        //println!("0: {:?}", bodies[idx_i].position);
        //println!("1: {:?}", bodies[idx_j].position);

        //let p = bodies[idx_i].velocity.dot(collision_normal) - bodies[idx_j].velocity.dot(collision_normal);
        //
        //println!("p: {:?}, collision_normal: {:?}", p, collision_normal);

        //bodies[idx_i].velocity = bodies[idx_i].velocity - p * collision_normal; 
        //bodies[idx_j].velocity = bodies[idx_j].velocity + p * collision_normal;

        //bodies[idx_i].prev_position = bodies[idx_i].position - bodies[idx_i].velocity;
        //bodies[idx_j].prev_position = bodies[idx_j].position - bodies[idx_j].velocity;
    }
}


#[cfg(test)]
mod tests {
    mod rect_rect {
        //use cgmath::Vector3;
        //use cgmath::Zero;
        //use crate::engine::physics_engine::collision::collision_body::CollisionBody;
        //use crate::engine::physics_engine::collision::collision_handler::CollisionHandler;
        //use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;

        
        //#[test]
        //fn horizontal_movement() {
        //    let mut bodies = vec![
        //        CollisionBody::rectangle(0, Vector3::new(25.0,0.0,0.0), Vector3::zero(),
        //            Vector3::zero(), Vector3::new(25.0,0.0,0.0), 100., 100.),

        //        CollisionBody::rectangle(1, Vector3::zero(), Vector3::zero(),
        //            Vector3::new(100.0,0.0,0.0), Vector3::new(100.0,0.0,0.0), 100., 100.),
        //    ]; 
        //    let solver = SimpleCollisionSolver::new();
        //    solver.handle_rect_rect_collision(&mut bodies, 0, 1);

        //    // TODO: Check if velocity is expected for circles
        //    //assert_eq!(bodies[0].velocity, Vector3::new(-25.0,0.0,0.0), "Wrong velocity for body 0");
        //    assert_eq!(bodies[0].acceleration, Vector3::zero(), "Wrong acceleration for body 0");
        //    assert_eq!(bodies[0].prev_position, Vector3::new(-25.0,0.0,0.0), "Wrong prev_position for body 0");
        //    assert_eq!(bodies[0].position, Vector3::zero(), "Wrong position for body 0");
        //    
        //    //assert_eq!(bodies[1].velocity, Vector3::zero(), "Wrong velocity for body 1");
        //    assert_eq!(bodies[1].acceleration, Vector3::zero(), "Wrong acceleration for body 1");
        //    assert_eq!(bodies[1].prev_position, Vector3::new(100.0,0.0,0.0), "Wrong prev_position for body 1");
        //    assert_eq!(bodies[1].position, Vector3::new(100.0,0.0,0.0), "Wrong position for body 1");

        //}
    }
}

