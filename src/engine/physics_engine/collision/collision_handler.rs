use crate::engine::physics_engine::util::equations::impulse_magnitude;

use super::rigid_body::{RigidBody, RigidBodyType};
use cgmath::{InnerSpace, MetricSpace, Vector3};

pub trait CollisionHandler {
    fn handle_circle_circle_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize);
    fn handle_circle_rect_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize);
    fn handle_rect_rect_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize);
}

pub struct IdentityCollisionSolver{}
impl IdentityCollisionSolver {
    pub fn new() -> Self { Self{} }
}

impl CollisionHandler for IdentityCollisionSolver {
    fn handle_rect_rect_collision(&self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) {}
    fn handle_circle_rect_collision(&self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) {}
    fn handle_circle_circle_collision(&self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) {}
}

pub struct SimpleCollisionSolver {}
impl SimpleCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for SimpleCollisionSolver {
    fn handle_circle_circle_collision(
        &self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize
    ) {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];
        let (radius_i, radius_j) = match (&body_i.body_type, &body_j.body_type) {
            (RigidBodyType::Circle { radius: ri }, RigidBodyType::Circle { radius: rj}) =>
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
        &self, bodies: &mut Vec<RigidBody>, circ_idx: usize, rect_idx: usize
    ) {
        let circle = &bodies[circ_idx];
        let radius = match circle.body_type {
            RigidBodyType::Circle { radius } => radius,
            _ => unreachable!(""),
        };

        let rect = &bodies[rect_idx];
        let (width, height) = match rect.body_type {
            RigidBodyType::Rectangle { width, height } => (width, height),
            _ => unreachable!(""),
        };

        let translated_circle_center = circle.position - rect.position; 
        //let rotated_circle_center = ... // TODO: Rotate circle center such rect is axis aligned
        // TODO: This should be rotated_circle_center instead of translated
        let temp_circle_center = translated_circle_center;
        let closest_point_on_rect_x = (-width/2.0).max(temp_circle_center.x.min(width/2.0));
        let closest_point_on_rect_y = (-height/2.0).max(temp_circle_center.y.min(height/2.0));
        let closest_point_on_rect = Vector3::new(closest_point_on_rect_x, closest_point_on_rect_y, 0.0);
        
        let distance2 = closest_point_on_rect.distance2(temp_circle_center);
        // Note: there is a corner case where the penetration depth is equal to the
        // radius of the circle. This will not cause an error but any computations
        // afterward are invalid.
        if distance2 >= radius.powi(2) {
            return;
        }
        debug_assert!(radius - distance2.sqrt() != 0.0,
            "Penetration depth equal to the radius the circle causes undefined behavior");
        let penetration_depth = radius - distance2.sqrt();

        let collision_normal_unit = (temp_circle_center - closest_point_on_rect).normalize();
        
        // TODO: Rotate the velocity to rectangle local space
        let temp_circle_vel = circle.velocity;
        
        let relative_vel = temp_circle_vel - rect.velocity;
        let relative_vel_along_norm = relative_vel.dot(collision_normal_unit);
        let mass_circle = circle.mass;
        let mass_rect = rect.mass;
        let c_r = 1.0;
        let impulse_magnitude = impulse_magnitude(relative_vel_along_norm, mass_circle, mass_rect, c_r);

        // resolve new velocities
        let new_temp_circle_vel = temp_circle_vel + (impulse_magnitude/mass_circle)*collision_normal_unit;
        let new_rect_vel = rect.velocity - (impulse_magnitude/mass_rect)*collision_normal_unit;
        // resolve penetration
        let new_temp_circle_center = temp_circle_center + ((penetration_depth*mass_rect)/(mass_circle+mass_rect))*collision_normal_unit; 
        let new_rect_center = rect.position - ((penetration_depth*mass_circle)/(mass_circle+mass_rect))*collision_normal_unit; 

        // TODO: Handle rotation (and maybe friction)

        // TODO: Rotate back to world space
        let new_circle_vel = new_temp_circle_vel;
        let new_circle_center = new_temp_circle_center + rect.position;
    
        bodies[circ_idx].position = new_circle_center;
        bodies[rect_idx].position = new_rect_center;

        bodies[circ_idx].velocity = new_circle_vel;
        bodies[rect_idx].velocity = new_rect_vel;

        bodies[circ_idx].prev_position = bodies[circ_idx].position - bodies[circ_idx].velocity;
        bodies[rect_idx].prev_position = bodies[rect_idx].position - bodies[rect_idx].velocity;
    }

    fn handle_rect_rect_collision(
        &self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize
    ) {
        //let body_i = &bodies[idx_i];
        //let body_j = &bodies[idx_j];
        //let ((wi,hi), (wj, hj)) = match (&body_i.body_type, &body_j.body_type) {
        //    (RigidBodyType::Rectangle { width: wi, height:hi },
        //     RigidBodyType::Rectangle { width: wj, height:hj }) => ((wi, hi), (wj,hj)), 
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
        //use crate::engine::physics_engine::collision::rigid_body::RigidBody;
        //use crate::engine::physics_engine::collision::collision_handler::CollisionHandler;
        //use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;

        
        //#[test]
        //fn horizontal_movement() {
        //    let mut bodies = vec![
        //        RigidBody::rectangle(0, Vector3::new(25.0,0.0,0.0), Vector3::zero(),
        //            Vector3::zero(), Vector3::new(25.0,0.0,0.0), 100., 100.),

        //        RigidBody::rectangle(1, Vector3::zero(), Vector3::zero(),
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
    mod circle_rect_collision {

        use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
        use crate::engine::physics_engine::collision::rigid_body::{RigidBodyBuilder, RigidBodyType};
        use crate::engine::util::zero;
        use super::super::CollisionHandler;

        macro_rules! handle_circle_rect_collision_tests {
            ($($name:ident: $bodies: expr, $expected_output: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let mut bodies = $bodies;
                        let expected_output = $expected_output;
                        let ch = SimpleCollisionSolver::new();
                        ch.handle_circle_rect_collision(&mut bodies, 0, 1);

                        assert_eq!(expected_output[0].position, bodies[0].position,
                            "Expected circle position {:?} but found {:?}", 
                            expected_output[0].position, bodies[0].position);
                        assert_eq!(expected_output[1].position, bodies[1].position,
                            "Expected rectangle position {:?} but found {:?}", 
                            expected_output[1].position, bodies[1].position);

                        assert_eq!(expected_output[0].velocity, bodies[0].velocity,
                            "Expected circle velocity {:?} but found {:?}", 
                            expected_output[0].velocity, bodies[0].velocity);
                        assert_eq!(expected_output[1].velocity, bodies[1].velocity,
                            "Expected rectangle velocity {:?} but found {:?}", 
                            expected_output[1].velocity, bodies[1].velocity);
                    }
                )*
            }
        }

        handle_circle_rect_collision_tests! {
            given_distance_between_bodies_is_zero_expect_no_collision_resolution:
                vec![
                    RigidBodyBuilder::default().id(0).velocity([10.,0.,0.])
                        .position([-100.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position(zero())
                        .body_type(RigidBodyType::Rectangle{ width: 100., height: 100.})
                        .build(),],
                vec![
                    RigidBodyBuilder::default().id(0).velocity([10.,0.,0.])
                        .position([-100.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position(zero())
                        .body_type(RigidBodyType::Rectangle{ width: 100., height: 100.})
                        .build(),]

            given_objects_have_collided_when_distance_is_zero_expect_each_object_move_half_penetration_depth:
                vec![
                    RigidBodyBuilder::default().id(0).velocity(zero())
                        .position([-50.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position(zero())
                        .body_type(RigidBodyType::Rectangle{ width: 80., height: 80.})
                        .build(),],
                vec![
                    RigidBodyBuilder::default().id(0).velocity(zero())
                        .position([-70.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position([20.,0.,0.])
                        .body_type(RigidBodyType::Rectangle{ width: 100., height: 100.})
                        .build(),]


            given_objects_collide_when_mass_is_equal_and_an_elastic_collision_expect_velocity_swap:
                vec![
                    RigidBodyBuilder::default().id(0).velocity([100.,0.,0.])
                        .position([-50.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position(zero())
                        .body_type(RigidBodyType::Rectangle{ width: 80., height: 80.})
                        .build(),],
                vec![
                    RigidBodyBuilder::default().id(0).velocity(zero())
                        .position([-70.0,0.,0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                    RigidBodyBuilder::default().id(1).velocity([100.,0.,0.])
                        .position([20.,0.,0.])
                        .body_type(RigidBodyType::Rectangle{ width: 100., height: 100.})
                        .build(),]


        }
    }
}

