use super::rigid_body::{RigidBody, RigidBodyType};
use super::sat::sat;
use crate::engine::physics_engine::util::rectangle_equations;
use crate::engine::{
    physics_engine::util::equations::{
        self, impulse_magnitude, post_collision_angular_velocity, post_collision_velocity,
    },
    util::fixed_float::fixed_float::FixedFloat,
};
use cgmath::{InnerSpace, MetricSpace, Vector3};

pub trait CollisionHandler {
    fn handle_circle_circle_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        idx_i: usize,
        idx_j: usize,
    ) -> bool;
    fn handle_circle_rect_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        idx_i: usize,
        idx_j: usize,
    ) -> bool;
    fn handle_rect_rect_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        idx_i: usize,
        idx_j: usize,
    ) -> bool;
}

pub struct IdentityCollisionSolver {}
impl IdentityCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for IdentityCollisionSolver {
    fn handle_circle_circle_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> bool {
        false
    }
    fn handle_circle_rect_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> bool {
        false
    }
    fn handle_rect_rect_collision(
        &self,
        _bodies: &mut Vec<RigidBody>,
        _idx_i: usize,
        _idx_j: usize,
    ) -> bool {
        false
    }
}

pub struct SimpleCollisionSolver {}
impl SimpleCollisionSolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionHandler for SimpleCollisionSolver {
    fn handle_circle_circle_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        idx_i: usize,
        idx_j: usize,
    ) -> bool {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];

        let (radius_i, radius_j) = match (&body_i.body_type, &body_j.body_type) {
            (RigidBodyType::Circle { radius: ri }, RigidBodyType::Circle { radius: rj }) => {
                (ri, rj)
            }
            (_, _) => unreachable!(),
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

        bodies[idx_i].position += 0.5 * collision_depth * correction_direction;
        bodies[idx_j].position -= 0.5 * collision_depth * correction_direction;

        let p = bodies[idx_i].velocity.dot(collision_normal)
            - bodies[idx_j].velocity.dot(collision_normal);
        bodies[idx_i].velocity = bodies[idx_i].velocity - p * collision_normal;
        bodies[idx_j].velocity = bodies[idx_j].velocity + p * collision_normal;

        bodies[idx_i].prev_position = bodies[idx_i].position - bodies[idx_i].velocity;
        bodies[idx_j].prev_position = bodies[idx_j].position - bodies[idx_j].velocity;

        return true;
    }

    fn handle_circle_rect_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        circ_idx: usize,
        rect_idx: usize,
    ) -> bool {
        let circle = &bodies[circ_idx];
        let radius = match circle.body_type {
            RigidBodyType::Circle { radius } => radius,
            _ => unreachable!(""),
        };

        let rect = &bodies[rect_idx];
        let closest_point_on_rect = rect.closest_point_on_rectangle(circle.position);
        let distance2: f32 = closest_point_on_rect.distance2(circle.position);

        // Note: there is a corner case where the penetration depth is equal to the
        // radius of the circle. This will not cause an error but any computations
        // afterward are invalid.
        if distance2 >= radius.powi(2) {
            return false;
        }

        let penetration_depth: f32 = FixedFloat::from(radius - distance2.sqrt()).into();

        if penetration_depth <= 0.0 {
            return false;
        }
        debug_assert!(penetration_depth >= 0.0,
            "Penetration depth less than or equal to the radius the circle causes undefined behavior");

        // Collision normal should always point towards object A (or circle in this case)
        let collision_normal_unit = (circle.position - closest_point_on_rect).normalize();

        let circle_center_to_p = closest_point_on_rect - circle.position;
        let rect_center_to_p = closest_point_on_rect - rect.position;
        let circle_vel_at_p =
            equations::total_velocity_at_point_p(&circle, &circle_center_to_p.into());
        let rect_vel_at_p = equations::total_velocity_at_point_p(&rect, &rect_center_to_p.into());

        let relative_vel_at_p = [
            circle_vel_at_p[0] - rect_vel_at_p[0],
            circle_vel_at_p[1] - rect_vel_at_p[1],
            circle_vel_at_p[2] - rect_vel_at_p[2],
        ];

        // If objects are moving away from each other, we do not consider a collision
        if equations::dot(&relative_vel_at_p, &collision_normal_unit.into()) > 0.0 {
            return false;
        }

        let c_r = 1.0;

        let impulse_magnitude = impulse_magnitude(
            c_r,
            &collision_normal_unit.into(),
            &circle_center_to_p.into(),
            &rect_center_to_p.into(),
            &relative_vel_at_p,
            &circle,
            &rect,
        );

        let normal_unit_array: [f32; 3] = collision_normal_unit.into();
        let new_rect_velocity = Vector3::from(post_collision_velocity(
            &normal_unit_array,
            -impulse_magnitude,
            &rect,
        ));
        let new_circ_velocity = Vector3::from(post_collision_velocity(
            &normal_unit_array,
            impulse_magnitude,
            &circle,
        ));

        let closest_point: [f32; 3] = closest_point_on_rect.into();
        let new_circ_angular_velocity = post_collision_angular_velocity(
            &normal_unit_array,
            &closest_point,
            impulse_magnitude,
            &circle,
        );
        let new_rect_angular_velocity = post_collision_angular_velocity(
            &normal_unit_array,
            &closest_point,
            -impulse_magnitude,
            &rect,
        );

        let circle_correction =
            (penetration_depth / (circle.mass + rect.mass)) * rect.mass * collision_normal_unit;
        let rect_correction =
            (penetration_depth / (circle.mass + rect.mass)) * circle.mass * -collision_normal_unit;
        let new_circle_center = circle.position + circle_correction;
        let new_rect_center = rect.position + rect_correction;

        debug_assert!(
            {
                let initial_linear_momentum =
                    circle.mass * circle.velocity + rect.mass * rect.velocity;
                let final_linear_momentum =
                    circle.mass * new_circ_velocity + rect.mass * new_rect_velocity;

                let initial_angular_momentum: f32 = FixedFloat::from(
                    circle.inertia() * circle.rotational_velocity
                        + rect.inertia() * rect.rotational_velocity
                        + equations::cross_2d(
                            &circle.position.into(),
                            &(circle.mass * circle.velocity).into(),
                        )
                        + equations::cross_2d(
                            &rect.position.into(),
                            &(rect.mass * rect.velocity).into(),
                        ),
                )
                .into();

                let final_angular_momentum: f32 = FixedFloat::from(
                    circle.inertia() * new_circ_angular_velocity
                        + rect.inertia() * new_rect_angular_velocity
                        + equations::cross_2d(
                            &circle.position.into(),
                            &(circle.mass * new_circ_velocity).into(),
                        )
                        + equations::cross_2d(
                            &rect.position.into(),
                            &(rect.mass * new_rect_velocity).into(),
                        ),
                )
                .into();
                // Note that we compare linear momentum component-wise
                initial_linear_momentum == final_linear_momentum
                    && initial_angular_momentum == final_angular_momentum
            },
            "Expected total momentum of collision to be preserved"
        );

        bodies[circ_idx].position = new_circle_center;
        bodies[rect_idx].position = new_rect_center;

        bodies[circ_idx].velocity = new_circ_velocity;
        bodies[rect_idx].velocity = new_rect_velocity;

        bodies[circ_idx].prev_position = bodies[circ_idx].position - bodies[circ_idx].velocity;
        bodies[rect_idx].prev_position = bodies[rect_idx].position - bodies[rect_idx].velocity;

        // We never update the rotation because we separate objects only by linear movement
        bodies[circ_idx].rotational_velocity = new_circ_angular_velocity;
        bodies[rect_idx].rotational_velocity = new_rect_angular_velocity;

        bodies[circ_idx].prev_rotation =
            bodies[circ_idx].rotation - bodies[circ_idx].rotational_velocity;
        bodies[rect_idx].prev_rotation =
            bodies[rect_idx].rotation - bodies[rect_idx].rotational_velocity;

        return true;
    }

    fn handle_rect_rect_collision(
        &self,
        bodies: &mut Vec<RigidBody>,
        idx_i: usize,
        idx_j: usize,
    ) -> bool {
        let body_i = &bodies[idx_i];
        let body_j = &bodies[idx_j];
        let ((wi, hi), (wj, hj)) = match (&body_i.body_type, &body_j.body_type) {
            (
                RigidBodyType::Rectangle {
                    width: wi,
                    height: hi,
                },
                RigidBodyType::Rectangle {
                    width: wj,
                    height: hj,
                },
            ) => ((wi, hi), (wj, hj)),
            (_, _) => unreachable!(),
        };

        // SAT get axii
        let axii_i = sat::sat_get_axii(&body_i);
        let axii_j = sat::sat_get_axii(&body_j);

        return false;
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

        use super::super::CollisionHandler;
        use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;
        use crate::engine::util::zero;

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
                        let output_0_pos: [f32; 3] = FixedFloatVector::from(output_circ.position).into();
                        assert_eq!(
                            expected_output_0_pos, output_0_pos, //$epsilon,
                            "Expected circle position {expected_output_0_pos:?} but found {output_0_pos:?}");

                        let expected_output_1_pos: [f32; 3] = expected_output_rect.position.into();
                        let output_1_pos: [f32; 3] = FixedFloatVector::from(output_rect.position).into();
                        assert_eq!(
                            expected_output_1_pos, output_1_pos, //$epsilon,
                            "Expected rectangle position {expected_output_1_pos:?} but found {output_1_pos:?}");

                        let expected_output_0_vel: [f32; 3] = expected_output_circ.velocity.into();
                        let output_0_vel: [f32; 3] = FixedFloatVector::from(output_circ.velocity).into();
                        assert_eq!(
                            expected_output_0_vel, output_0_vel, //$epsilon,
                            "Expected circle velocity {expected_output_0_vel:?} but found {output_0_vel:?}");

                        let expected_output_1_vel: [f32; 3] = expected_output_rect.velocity.into();
                        let output_1_vel: [f32; 3] = FixedFloatVector::from(output_rect.velocity).into();
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

                    given_circle_have_collided_when_distance_is_zero_expect_each_object_move_half_penetration_depth:
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

                    given_circle_collide_with_rect_when_mass_is_equal_and_an_elastic_collision_expect_velocity_swap:
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

                    given_circle_collide_when_circle_is_at_origo_and_moves_diagonally_towards_aabb_rect_expect_bounce:
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

                    given_circle_collide_when_rect_is_at_origo_and_circle_moves_diagonally_towards_aabb_rect_expect_bounce:
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

                    given_circle_collide_when_rect_is_rotated_90_degrees_expect_bounce:
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

                    given_rect_collide_when_circle_is_not_moving_expect_velocity_transfer:
                        vec![
                            RigidBodyBuilder::default().id(0)
                                .position([50.,0.,0.])
                                .velocity([0.,0.,0.])
                                .body_type(RigidBodyType::Circle { radius: 5.0 })
                                .mass(1.0)
                                .build(),
                            RigidBodyBuilder::default().id(1)
                                .position([50., 8.,0.])
                                .velocity([0.,-2.,0.])
                                .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                                .mass(1.0)
                                .build()],
                        vec![
                            RigidBodyBuilder::default().id(0)
                                .position([50.,-1.,0.])
                                .velocity([0.,-2.,0.])
                                .build(),
                            RigidBodyBuilder::default().id(1)
                                .position([50.,9.,0.])
                                .velocity([0.,0.,0.])
                                .build()]

                 given_circle_moves_along_y_axis_when_collision_occurs_with_rectangle_offset_from_center_expect_transitional_angular_momentum_to_transfer_into_transitional_and_rotational_angular_momentum:
                         vec![
                             RigidBodyBuilder::default().id(0)
                                 .position([-100.0,-4.0,0.])
                                 .velocity([0., 5.,0.])
                                 .body_type(RigidBodyType::Circle { radius: 5.0 })
                                 .build(),
                             RigidBodyBuilder::default().id(1)
                                 .position([0.,5.,0.])
                                 .velocity([0.,0.,0.])
                                 .body_type(RigidBodyType::Rectangle { width: 300., height: 10.})
                                 .build()],
                         vec![
                             RigidBodyBuilder::default().id(0)
                                 .position([-100.0,-4.5,0.])
                                 .velocity([0.,1.999,0.])
                                 .build(),
                             RigidBodyBuilder::default().id(1)
                                 .position([0.,5.5,0.])
                                 .velocity([0.,3.001,0.])
                                 .build()]

                 given_rect_rotates_at_origo_when_collision_with_non_moving_circle_expect_rotational_energy_translate_to_linear_and_rotation:
                         vec![
                             RigidBodyBuilder::default().id(0)
                                 .position([-400.,-3.,0.])
                                 .velocity([0.,0.,0.])
                                 .body_type(RigidBodyType::Circle { radius: 5.0 })
                                 .build(),
                             RigidBodyBuilder::default().id(1)
                                 .position([0.,5.,0.])
                                 .velocity([0.,0.,0.])
                                 .rotational_velocity(std::f32::consts::PI/120.0)
                                 .body_type(RigidBodyType::Rectangle { width: 1000., height: 10.})
                                 .build()],
                         vec![
                             RigidBodyBuilder::default().id(0)
                                 .position([-400.,-4.,0.])
                                 .velocity([0.,-5.343,0.])
                                 .build(),
                             RigidBodyBuilder::default().id(1)
                                 .position([0.,6.,0.])
                                 .velocity([0.,5.343,0.])
                                 .build()]
        }

        #[test]
        fn given_circle_collide_when_rect_is_rotated_45_degrees_and_circle_moves_perpendicular_towards_rect_expect_penetration_depth_to_be_resolved(
        ) {
            let circle = RigidBodyBuilder::default()
                .id(0)
                .position([-144., 144., 0.])
                .velocity([4., -4., 0.])
                .body_type(RigidBodyType::Circle { radius: 50.0 })
                .build();
            let rectangle = RigidBodyBuilder::default()
                .id(1)
                .position([0., 150., 0.])
                .velocity([0., 0., 0.])
                .rotation(std::f32::consts::PI / 4.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 500.,
                    height: 100.,
                })
                .build();

            let mut bodies = vec![circle, rectangle];
            let ch = SimpleCollisionSolver::new();

            // Perform initial collision test
            ch.handle_circle_rect_collision(&mut bodies, 0, 1);

            let expected_output_circ = bodies[0].clone();
            let expected_output_rect = bodies[1].clone();

            // Perform second collision test and expect no change
            ch.handle_circle_rect_collision(&mut bodies, 0, 1);
            let output_circ = &bodies[0];
            let output_rect = &bodies[1];

            let expected_output_0_pos: [f32; 3] = expected_output_circ.position.into();
            let output_0_pos: [f32; 3] = output_circ.position.into();
            assert_eq!(
                expected_output_0_pos,
                output_0_pos, //$epsilon,
                "Expected circle position {expected_output_0_pos:?} but found {output_0_pos:?}"
            );

            let expected_output_1_pos: [f32; 3] = expected_output_rect.position.into();
            let output_1_pos: [f32; 3] = output_rect.position.into();
            assert_eq!(
                expected_output_1_pos,
                output_1_pos, //$epsilon,
                "Expected rectangle position {expected_output_1_pos:?} but found {output_1_pos:?}"
            );

            let expected_output_0_vel: [f32; 3] = expected_output_circ.velocity.into();
            let output_0_vel: [f32; 3] = output_circ.velocity.into();
            assert_eq!(
                expected_output_0_vel,
                output_0_vel, //$epsilon,
                "Expected circle velocity {expected_output_0_vel:?} but found {output_0_vel:?}"
            );

            let expected_output_1_vel: [f32; 3] = expected_output_rect.velocity.into();
            let output_1_vel: [f32; 3] = output_rect.velocity.into();
            assert_eq!(
                expected_output_1_vel, output_1_vel,
                "Expected rectangle velocity {expected_output_1_vel:?} but found {output_1_vel:?}"
            );
        }
    }
}
