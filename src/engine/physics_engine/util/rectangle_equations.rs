use crate::engine::{physics_engine::collision::rigid_body::{RigidBody, RigidBodyType}, util::fixed_float::fixed_float_vector::FixedFloatVector};

use super::equations::{self, magnitude2};

/// Returns the moment of inertia for a solid rectangle rotating around its center
pub fn inertia(height:f32, width:f32, mass:f32) -> f32 {
    (mass/12.0)*(height.powi(2) + width.powi(2))
}

/// Returns the left-, right-, top- and bottom-most points of a rotated rectangle
pub fn cardinals(center: &[f32;3], width: f32, height:f32, rotation: f32) -> [[f32;3];4]{
    let top_left  = [-width/2.0,  height/2.0, 0.0];
    let top_right = [ width/2.0,  height/2.0, 0.0];
    let bot_right = [ width/2.0, -height/2.0, 0.0];
    let bot_left  = [-width/2.0, -height/2.0, 0.0];

    let top_left_rot = equations::rotate_z(&top_left, rotation);
    let top_right_rot = equations::rotate_z(&top_right, rotation);
    let bot_right_rot = equations::rotate_z(&bot_right, rotation);
    let bot_left_rot = equations::rotate_z(&bot_left, rotation);
    
    let top_left_offset  = [center[0] + top_left_rot[0],  center[1] + top_left_rot[1],  0.0];
    let top_right_offset = [center[0] + top_right_rot[0], center[1] + top_right_rot[1], 0.0];
    let bot_right_offset = [center[0] + bot_right_rot[0], center[1] + bot_right_rot[1], 0.0];
    let bot_left_offset  = [center[0] + bot_left_rot[0],  center[1] + bot_left_rot[1],  0.0];

    let corners = [top_left_offset, top_right_offset, bot_right_offset, bot_left_offset];
    let left_most = corners.iter().min_by(|a,b| a[0].partial_cmp(&b[0]).unwrap()).unwrap();
    let right_most = corners.iter().max_by(|a,b| a[0].partial_cmp(&b[0]).unwrap()).unwrap();
    let top_most = corners.iter().max_by(|a,b| a[1].partial_cmp(&b[1]).unwrap()).unwrap();
    let bot_most = corners.iter().min_by(|a,b| a[1].partial_cmp(&b[1]).unwrap()).unwrap();

    return [
        FixedFloatVector::from(*left_most).into(), 
        FixedFloatVector::from(*right_most).into(),
        FixedFloatVector::from(*top_most).into(),
        FixedFloatVector::from(*bot_most).into()];
}

//pub fn apply_impulse(
//    coll_normal: &[f32;3], collision_point: &[f32;3], body: &RigidBody, impulse: f32,
//    r_cp: &[f32;3]
//) -> ([f32;3], f32) {
//    let j = [
//        coll_normal[0] * impulse,
//        coll_normal[1] * impulse,
//        coll_normal[2] * impulse,
//    ];
//    let j_linear = magnitude2(&j) / (2.0*body.mass);
//    let j_angular = (magnitude2(&j)*magnitude2(&r_cp)) / (2.0*body.inertia());
//
//    let new_linear_velocity = equations::post_collision_velocity(coll_normal, j_linear, body);
//    let new_angular_velocity = equations::post_collision_angular_velocity(coll_normal, collision_point, j_angular, body);
//
//    println!("j_linear: {j_linear}, j_angular: {j_angular}");
//    println!("new_linear_vel: {new_linear_velocity:?}");
//    println!("new_angular_vel: {new_angular_velocity}");
//
//    return (new_linear_velocity, new_angular_velocity);
//}

//pub fn corners(body: &RigidBody) -> [[f32;3];4] {
//    let (width, height) = match body.body_type {
//        RigidBodyType::Rectangle { width, height } => (width, height),
//        _ => panic!("Expected rectangle body"),
//    };
//    [
//        equations::rotate_z(&[body.position.x - width/2.0, body.position.y + height/2.0, 0.0], body.rotation),// Top left
//        equations::rotate_z(&[body.position.x + width/2.0, body.position.y + height/2.0, 0.0], body.rotation),// Top right
//        equations::rotate_z(&[body.position.x + width/2.0, body.position.y - height/2.0, 0.0], body.rotation),// Bottom right
//        equations::rotate_z(&[body.position.x - width/2.0, body.position.y - height/2.0, 0.0], body.rotation),// Bottom left
//    ]
//}

pub fn sat_get_axii(body: &RigidBody) -> [[f32;3];2] {
    let (width, height) = match body.body_type {
        RigidBodyType::Rectangle { width, height } => (width, height),
        _ => panic!("Expected rectangle body"),
    };

    let top_left = equations::rotate_z(
        &[body.position.x - width/2.0, body.position.y + height/2.0, 0.0], body.rotation);
    let top_right = equations::rotate_z(
        &[body.position.x + width/2.0, body.position.y + height/2.0, 0.0], body.rotation);
    let bot_right = equations::rotate_z(
        &[body.position.x + width/2.0, body.position.y - height/2.0, 0.0], body.rotation);

    let axis1 = [bot_right[0]-top_right[0],bot_right[1]-top_right[1],bot_right[2]-top_right[2]];
    let axis2 = [top_right[0]-top_left[0],top_right[1]-top_left[1],top_right[2]-top_left[2]];
    let mut normal1 = equations::perpendicular_2d(&axis1);
    let mut normal2 = equations::perpendicular_2d(&axis2);
    equations::normalize(&mut normal1);
    equations::normalize(&mut normal2);
    [normal1, normal2]
}

#[cfg(test)]
mod rectangle_equations_test {
    mod cardinals {
        use super::super::cardinals;
        macro_rules! cardinals_test {
            ($($name:ident: $center: expr, $width: expr, $height: expr, $rotation: expr,
                $expected_left: expr, $expected_right: expr, $expected_top:expr, $expected_bot: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let exp_left = $expected_left;
                        let exp_right = $expected_right;
                        let exp_top = $expected_top;
                        let exp_bot = $expected_bot;
                        let outputs = cardinals($center, $width, $height, $rotation);
                        let out_left = outputs[0];
                        let out_right = outputs[1];
                        let out_top = outputs[2];
                        let out_bot = outputs[3];
                        assert_eq!(exp_left, out_left, "Expected left most {exp_left:?} found {out_left:?}");
                        assert_eq!(exp_right, out_right, "Expected right most {exp_right:?} found {out_right:?}");
                        assert_eq!(exp_top, out_top, "Expected top most {exp_top:?} found {out_top:?}");
                        assert_eq!(exp_bot, out_bot, "Expected bottom most {exp_bot:?} found {out_bot:?}");
                    }
                )*
            }
        }
    
        cardinals_test! {
            given_rect_when_aabb_and_no_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, 0.0,
                [-1.0,1.0,0.0],[1.0,-1.0,0.0],[1.0,1.0,0.0],[1.0,-1.0,0.0]
            given_rect_when_aabb_and_90_degrees_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, std::f32::consts::PI/2.0,
                [-1.0,-1.0,0.0],[1.0,-1.0,0.0],[1.0,1.0,0.0],[-1.0,-1.0,0.0]
            given_rect_when_aabb_and_30_degrees_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, std::f32::consts::PI/6.,
                [-1.366,0.366,0.0],[1.366,-0.366,0.0],[0.366,1.366,0.0],[-0.366,-1.366,0.0]
            
            given_rect_when_aabb_and_30_degrees_rotation_and_offset_expect_corners:
                &[1.,0.,0.], 2.0, 2.0, std::f32::consts::PI/6.,
                [-0.366,0.366,0.0],[2.366,-0.366,0.0],[1.366,1.366,0.0],[0.634,-1.366,0.0]

        }
    }

    mod sat_get_axii {
        use super::super::sat_get_axii;
        use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;
        use crate::engine::physics_engine::collision::rigid_body::{RigidBodyBuilder, RigidBodyType};
        macro_rules! sat_get_axii_tests {
            ($($name:ident: $body: expr, $expected_axis1: expr, $expected_axis2: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let exp1 = $expected_axis1;
                        let exp2 = $expected_axis2;
                        let axii = sat_get_axii(&$body);
                        let ax1: [f32;3] = FixedFloatVector::from(axii[0]).into();
                        let ax2: [f32;3] = FixedFloatVector::from(axii[1]).into();
                        assert_eq!(exp1, ax1, "Expected first normal to be {exp1:?} but found {ax1:?}");
                        assert_eq!(exp2, ax2, "Expected second normal to be {exp2:?} but found {ax2:?}");
                    }
                )*
            }
        }

        sat_get_axii_tests! {
            given_rect_with_no_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([0.0,0.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
            given_rect_is_offset_with_no_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
            given_rect_is_offset_with_45_degree_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]
            given_rect_is_offset_with_45_degree_rotation_and_uneven_height_and_width_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 30., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]

        }
    }

    mod apply_impulse {
        //use cgmath::Vector3;

        //use crate::engine::physics_engine::collision::rigid_body::{RigidBodyBuilder, RigidBodyType};
        //use crate::engine::physics_engine::util::equations;
        //use super::super::apply_impulse;

        //#[test]
        //fn apply_impulse_test(){
        //    let impulse = 5.343;
        //    let mut rectangle = RigidBodyBuilder::default().id(1)
        //        .position([0.,5.,0.])
        //        .mass(1.0)
        //        .velocity([0.,0.,0.])
        //        .rotational_velocity(std::f32::consts::PI/120.0)
        //        .body_type(RigidBodyType::Rectangle { width: 1000., height: 10.})
        //        .build();
        //        
        //    let collision_point = [-400.0, 0.0, 0.0];
        //    let collision_normal = [0.0,-1.0,0.0];
        //    let r_rp = [-400.0, -5.0, 0.0];
        //    // TODO: Angular momentum should not be preserved because the impulse apply 
        //    // applies a torque
        //    // TODO: System total momentun (linear + angular) should be preserved
        //    // TODO: Kinetic energy needs to be preserved (elastic collision)
        //    let initial_kinetic_energy = 
        //                    equations::translational_kinetic_energy(&rectangle) + 
        //                    equations::rotational_kinetic_energy(&rectangle);
        //   
        //    let (new_linear_vel, new_angular_vel) = apply_impulse(
        //        &collision_normal, &collision_point, &rectangle, -impulse, &r_rp);

        //    rectangle.velocity = Vector3::from(new_linear_vel);
        //    rectangle.rotational_velocity = new_angular_vel;

        //    let resulting_kinetic_energy = 
        //                    equations::translational_kinetic_energy(&rectangle) + 
        //                    equations::rotational_kinetic_energy(&rectangle);
        //    
        //    assert_eq!(initial_kinetic_energy, resulting_kinetic_energy,
        //        "Expected the kinetic energy to be equal before and after collision. Before: {initial_kinetic_energy} and after: {resulting_kinetic_energy}");

        //}
    }
}

