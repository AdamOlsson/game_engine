use crate::engine::physics_engine::util::equations::{self, impulse_magnitude};

use super::rigid_body::{RigidBody, RigidBodyType};
use cgmath::{InnerSpace, MetricSpace, Vector3};

pub trait CollisionHandler {
    fn handle_circle_circle_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize) -> bool;
    fn handle_circle_rect_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize) -> bool;
    fn handle_rect_rect_collision(&self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize) -> bool;
}

pub struct IdentityCollisionSolver{}
impl IdentityCollisionSolver {
    pub fn new() -> Self { Self{} }
}

impl CollisionHandler for IdentityCollisionSolver {
    fn handle_circle_circle_collision(
        &self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) -> bool { false }
    fn handle_circle_rect_collision(
        &self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) -> bool { false }
    fn handle_rect_rect_collision(
        &self, _bodies: &mut Vec<RigidBody>, _idx_i: usize, _idx_j: usize) -> bool { false }
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
    ) -> bool {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];

        let (radius_i, radius_j) = match (&body_i.body_type, &body_j.body_type) {
            (RigidBodyType::Circle { radius: ri }, RigidBodyType::Circle { radius: rj} ) =>
                (ri, rj),
            (_, _) => unreachable!()
        };

        let dist = body_i.position.distance(body_j.position);
        if dist >= (radius_i + radius_j) {
            return false;
        }

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

        return true;
    }

    fn handle_circle_rect_collision(
        &self, bodies: &mut Vec<RigidBody>, circ_idx: usize, rect_idx: usize
    ) -> bool {
        let circle = &bodies[circ_idx];
        let radius = match circle.body_type {
            RigidBodyType::Circle { radius } => radius,
            _ => unreachable!(""),
        };

        let rect = &bodies[rect_idx];
        let closest_point_on_rect = rect.closest_point_on_rectangle(circle.position);
        let distance2 = closest_point_on_rect.distance2(circle.position);

        println!("closest_point_on_rect: {closest_point_on_rect:?}");
        println!("circle: {circle}");
        println!("rect: {rect}");
        println!("distance: {}", distance2.sqrt());
        // Note: there is a corner case where the penetration depth is equal to the
        // radius of the circle. This will not cause an error but any computations
        // afterward are invalid.
        if distance2 >= radius.powi(2) {
            return false;
        }
        let penetration_depth = radius - distance2.sqrt();
        debug_assert!(penetration_depth >= 0.0,
            "Penetration depth less than or equal to the radius the circle causes undefined behavior");

        println!("penetration_depth: {penetration_depth}");

        println!("closest_point_on_rect: {closest_point_on_rect:?}");

        let collision_normal_unit = (closest_point_on_rect - circle.position).normalize();
        println!("collision_normal_unit: {collision_normal_unit:?}"); 

        let relative_vel = circle.velocity - rect.velocity;
        let relative_vel_along_norm = relative_vel.dot(collision_normal_unit);
        let mass_circle = circle.mass;
        let mass_rect = rect.mass;
        let c_r = 1.0;
        let impulse_magnitude = impulse_magnitude(relative_vel_along_norm, mass_circle, mass_rect, c_r);
        println!("relative_vel: {relative_vel:?}");
        println!("relative_vel_along_norm: {relative_vel_along_norm}");
        println!("impulse_magnitude: {impulse_magnitude}");
            
        // resolve new velocities
        let new_rect_velocity = rect.velocity - (impulse_magnitude/mass_rect)*collision_normal_unit;
        let circ_vel_size_adjusted = circle.velocity + (impulse_magnitude/mass_circle)*collision_normal_unit;
        // reflection
        let new_circ_velocity = circ_vel_size_adjusted - 
            2.0*(circ_vel_size_adjusted.dot(collision_normal_unit))*collision_normal_unit;

        let circle_correction = (penetration_depth/(mass_circle+mass_rect))*mass_rect*-collision_normal_unit;
        let rect_correction = (penetration_depth/(mass_circle+mass_rect))*mass_circle*collision_normal_unit;
        let new_circle_center = circle.position + circle_correction; 
        let new_rect_center = rect.position + rect_correction; 

        println!("circle_correction: {circle_correction:?}");
        println!("rect_correction: {rect_correction:?}");
        
        println!("new_circle_center: {new_circle_center:?}");
        println!("new_rect_center: {new_rect_center:?}");
        println!("");
        // TODO: Handle rotation (and maybe friction)

        bodies[circ_idx].position = new_circle_center;
        bodies[rect_idx].position = new_rect_center;

        bodies[circ_idx].velocity = new_circ_velocity;
        bodies[rect_idx].velocity = new_rect_velocity;

        bodies[circ_idx].prev_position = bodies[circ_idx].position - bodies[circ_idx].velocity;
        bodies[rect_idx].prev_position = bodies[rect_idx].position - bodies[rect_idx].velocity;

        return true;
    }

    fn handle_rect_rect_collision(
        &self, bodies: &mut Vec<RigidBody>, idx_i: usize, idx_j: usize
    ) -> bool {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];
        let ((wi,hi), (wj, hj)) = match (&body_i.body_type, &body_j.body_type) {
            (RigidBodyType::Rectangle { width: wi, height:hi },
             RigidBodyType::Rectangle { width: wj, height:hj }) => ((wi, hi), (wj,hj)), 
            (_, _) => unreachable!()
        };   
       
        if body_i.position.x + wi/2.0 >= body_j.position.x &&
                                body_i.position.x <= body_j.position.x + wj/2.0 &&
                                body_i.position.y + hi/2.0 >= body_j.position.y &&
                                body_i.position.y <= body_j.position.y + hj/2.0 {
            return false;
        }
        return true;
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

                        let expected_output_circ = &expected_output[0];
                        let expected_output_rect = &expected_output[1];
                        let output_circ = &bodies[0];
                        let output_rect = &bodies[1];

                        let expected_output_0_pos: [f32; 3] = expected_output_circ.position.into();
                        let output_0_pos: [f32; 3] = output_circ.position.into();
                        assert_eq!(
                            expected_output_0_pos, output_0_pos, //$epsilon,
                            "Expected circle position {expected_output_0_pos:?} but found {output_0_pos:?}");

                        let expected_output_1_pos: [f32; 3] = expected_output_rect.position.into();
                        let output_1_pos: [f32; 3] = output_rect.position.into();
                        assert_eq!(
                            expected_output_1_pos, output_1_pos, //$epsilon,
                            "Expected rectangle position {expected_output_1_pos:?} but found {output_1_pos:?}");

                        let expected_output_0_vel: [f32; 3] = expected_output_circ.velocity.into();
                        let output_0_vel: [f32; 3] = output_circ.velocity.into();
                        assert_eq!(
                            expected_output_0_vel, output_0_vel, //$epsilon,
                            "Expected circle velocity {expected_output_0_vel:?} but found {output_0_vel:?}");
                        
                        let expected_output_1_vel: [f32; 3] = expected_output_rect.velocity.into();
                        let output_1_vel: [f32; 3] = output_rect.velocity.into();
                        assert_eq!(
                            expected_output_1_vel, output_1_vel,
                            "Expected rectangle velocity {expected_output_1_vel:?} but found {output_1_vel:?}");
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
                        .position([-100.0,0.,0.]).build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position(zero()).build(),]

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
                        .position([-70.0,0.,0.]).build(),
                    RigidBodyBuilder::default().id(1).velocity(zero())
                        .position([20.,0.,0.]).build(),]

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
                        .position([-70.0,0.,0.]).build(),
                    RigidBodyBuilder::default().id(1).velocity([100.,0.,0.])
                        .position([20.,0.,0.]).build(),]

            given_objects_collide_when_circle_is_at_origo_and_moves_diagonally_towards_aabb_rect_expect_bounce:
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position([0.,0.,0.])
                        .velocity([10.0,-10.0,0.])
                        .body_type(RigidBodyType::Circle { radius: 5.0 })
                        .mass(1.0)
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([10.,0.,0.])
                        .velocity([0.,0.,0.])
                        .body_type(RigidBodyType::Rectangle { width: 15., height: 100.})
                        .mass(1.0)
                        .build()],
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position([-1.25,0.,0.])
                        .velocity([0.,-10.,0.])
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([11.25,0.,0.])
                        .velocity([10.,0.,0.])
                        .build()]

            given_objects_collide_when_rect_is_at_origo_and_circle_moves_diagonally_towards_aabb_rect_expect_bounce:
                vec![
                    RigidBodyBuilder::default().id(0).position([-10.,0.,0.])
                        .velocity([10.0,-10.0,0.]).mass(1.0)
                        .body_type(RigidBodyType::Circle { radius: 5.0 })
                        .build(),
                    RigidBodyBuilder::default().id(1).position([0.,0.,0.])
                        .velocity([0.,0.,0.]).mass(1.0)
                        .body_type(RigidBodyType::Rectangle { width: 15., height: 100.})
                        .build()],
                vec![
                    RigidBodyBuilder::default().id(0).position([-11.25,0.,0.])
                        .velocity([0.,-10.,0.]).build(),
                    RigidBodyBuilder::default().id(1).position([1.25,0.,0.])
                        .velocity([10.,0.,0.]).build(),]

            given_object_collide_when_rect_is_rotated_90_degrees_expect_bounce:
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position([-7.5,0.,0.])
                        .velocity([10.0, 0.0,0.])
                        .body_type(RigidBodyType::Circle { radius: 5.0 })
                        .mass(1.0)
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([0.,0.,0.])
                        .velocity([0.,0.,0.])
                        .rotation(std::f32::consts::PI/2.0)
                        .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                        .mass(1.0)
                        .build()],
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position([-8.75,0.,0.])
                        .velocity([0.,0.,0.])
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([1.25, 0.,0.])
                        .velocity([10.,0.,0.])
                        .build()]

            given_object_collide_when_rect_is_rotated_45_degrees_and_circle_moves_perpendicular_towards_rect_expect_full_velocity_transfer:
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position([-7.,7.,0.])
                        .velocity([5.0,-5.0,0.])
                        .body_type(RigidBodyType::Circle { radius: 5.0 })
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([0.,0.,0.])
                        .velocity([0.,0.,0.])
                        .rotation(std::f32::consts::PI/4.0)
                        .body_type(RigidBodyType::Rectangle { width: 30., height: 10.})
                        .build()],
                vec![
                    RigidBodyBuilder::default().id(0)
                        .position( [-7.035767, 7.035767, 0.0])
                        .velocity([0.,0.,0.])
                        .build(),
                    RigidBodyBuilder::default().id(1)
                        .position([0.035766996, -0.035766996, 0.0])
                        .velocity([5.,-5.,0.])
                        .build()]

        }
    }
}

