use crate::engine::physics_engine::collision::rigid_body::RigidBody;
use crate::engine::util::fixed_float::fixed_float::FixedFloat;

pub fn inelastic_collision_1d(
    m_a: f32,  // mass of the first object
    m_b: f32,  // mass of the second object
    u_a: f32,  // initial velocity of the first object
    u_b: f32,  // initial velocity of the second object
    c_r: f32,  // coefficient of restitution
) -> (f32, f32) {
    let v_a = (c_r * m_b * (u_b - u_a) + m_a * u_a + m_b * u_b) / (m_a + m_b);
    let v_b = (c_r * m_a * (u_a - u_b) + m_a * u_a + m_b * u_b) / (m_a + m_b);
    (v_a, v_b)
}

pub fn perpendicular_2d(v: &[f32;3]) -> [f32;3] {
    [-v[1], v[0], v[2]]
}

pub fn magnitude(v: &[f32;3]) -> f32 {
    (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt()
}

pub fn magnitude2(v: &[f32;3]) -> f32 {
    v[0].powi(2) + v[1].powi(2) + v[2].powi(2)
}

pub fn impulse_magnitude(
    e: f32, coll_normal: &[f32;3], collision_point: &[f32;3],
    body_a: &RigidBody, body_b: &RigidBody,
) -> f32 {

    let r_ap = [
        collision_point[0] - body_a.position.x,
        collision_point[1] - body_a.position.y,
        collision_point[2] - body_a.position.z,
    ];
    let r_bp = [
        collision_point[0] - body_b.position.x,
        collision_point[1] - body_b.position.y,
        collision_point[2] - body_b.position.z,
    ];
    let r_ap_perp = perpendicular_2d(&r_ap);
    let r_bp_perp = perpendicular_2d(&r_bp);

    let rel_vel = [
        (body_a.velocity.x + body_a.rotational_velocity*r_ap_perp[0]) -
            (body_b.velocity.x + body_b.rotational_velocity*r_bp_perp[0]),

        (body_a.velocity.y + body_a.rotational_velocity*r_ap_perp[1]) -
            (body_b.velocity.y + body_b.rotational_velocity*r_bp_perp[1]),

        (body_a.velocity.z + body_a.rotational_velocity*r_ap_perp[2]) -
            (body_b.velocity.z + body_b.rotational_velocity*r_bp_perp[2]),
    ];

    let nom = -(1.0+e)*dot(&rel_vel.into(), &coll_normal);
    let denom_term_1 = dot(coll_normal, coll_normal) * (1.0/body_a.mass) + (1.0/body_b.mass);
    let denom_term_2 = dot(&r_ap_perp, &coll_normal).powi(2) / body_a.inertia();
    let denom_term_3 = dot(&r_bp_perp, &coll_normal).powi(2) / body_b.inertia();
  
    return nom/(denom_term_1 + denom_term_2 + denom_term_3);
}

pub fn dot(v1: &[f32; 3], v2: &[f32; 3]) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn post_collision_velocity(
    coll_normal: &[f32;3], impulse:f32, body: &RigidBody 
) -> [f32;3]{
    [
        body.velocity.x + (impulse/body.mass)*coll_normal[0],
        body.velocity.y + (impulse/body.mass)*coll_normal[1],
        body.velocity.z + (impulse/body.mass)*coll_normal[2],
    ]
}

pub fn post_collision_angular_velocity(
    coll_normal: &[f32;3], collision_point: &[f32;3], impulse: f32, body: &RigidBody
) -> f32 { 
    let center_coll_point_perp = perpendicular_2d(&[
        collision_point[0] - body.position.x,
        collision_point[1] - body.position.y,
        collision_point[2] - body.position.z,
    ]);
    let scaled_norm = [
        coll_normal[0]*impulse, coll_normal[1]*impulse, coll_normal[2]*impulse];

    let new_angular_velocity = 
        body.rotational_velocity + dot(&center_coll_point_perp, &scaled_norm) / body.inertia();

    return new_angular_velocity;
}

pub fn cross_2d(a: &[f32;3], b: &[f32;3]) -> f32 {
    a[0]*b[1] - a[1]*b[0]
}

pub fn rotate_z(v: &[f32; 3], theta: f32) -> [f32; 3] {
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    [
        FixedFloat::from(v[0] * cos_theta - v[1] * sin_theta).into(),
        FixedFloat::from(v[0] * sin_theta + v[1] * cos_theta).into(),
        FixedFloat::from(v[2]).into(), // z remains unchanged
    ]
}

pub fn normalize(v: &mut [f32; 3]) {
    let magnitude = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
    if magnitude != 0.0 {
        v[0] /= magnitude;
        v[1] /= magnitude;
        v[2] /= magnitude;
    }
}

pub fn translational_kinetic_energy(body: &RigidBody) -> f32 {
    0.5*body.mass*magnitude2(&body.velocity.into())
}

pub fn rotational_kinetic_energy(body: &RigidBody) -> f32 {
    0.5*body.inertia()*body.rotational_velocity.powi(2)
}

pub fn angular_momentum(body: & RigidBody) -> f32 {
    body.inertia()*body.rotational_velocity
}

pub fn linear_momentum(body: &RigidBody) -> [f32;3] {
    [
        body.mass*body.velocity.x,
        body.mass*body.velocity.y,
        body.mass*body.velocity.z,
    ]
}

#[cfg(test)]
mod test {
    macro_rules! rotate_z_tests {
        ($($name:ident: $expected: expr, $output: expr)*) => {
            $(
                #[test]
                fn $name() {
                    let e = $expected;
                    let o = $output;
                    assert_eq!($expected, $output, "Expected {e:?} found {o:?}"); 
                }
            )*
        }
    }

    use crate::engine::{physics_engine::{collision::rigid_body::{RigidBodyBuilder, RigidBodyType}, util::equations::{cross_2d, post_collision_angular_velocity, post_collision_velocity}}, util::fixed_float::{fixed_float::FixedFloat, fixed_float_vector::FixedFloatVector}};

    use super::{impulse_magnitude, rotate_z};
    use std::f32::consts::PI;

    rotate_z_tests! {
        given_x_unit_vector_direction_when_rotated_90_degrees_expect_y_unit_vector:
            [0.0,1.0,0.0], rotate_z(&[1.0,0.0,0.0], PI/2.0)
        given_y_unit_vector_direction_when_rotated_90_degrees_expect_negative_x_unit_vector:
            [-1.0,0.0,0.0], rotate_z(&[0.0,1.0,0.0], PI/2.0)
        given_negative_x_unit_vector_direction_when_rotated_90_degrees_expect_negative_y_unit_vector:
            [0.0,-1.0,0.0], rotate_z(&[-1.0,0.0,0.0], PI/2.0)
        given_negative_y_unit_vector_direction_when_rotated_90_degrees_expect_x_unit_vector:
            [1.0,0.0,0.0], rotate_z(&[0.0,-1.0,0.0], PI/2.0)
        given_x_unit_vector_when_rotated_45_degrees_expect_vector_rotated:
            [0.707,0.707,0.0], rotate_z(&[1.0,0.0,0.0], PI/4.0)
        given_vector_90_degrees_counter_clockwise_expect_vector_rotated:
            [-75.0,-220.0,0.0], rotate_z(&[220.0,-75.0,0.0], -PI/2.0)

    }

    #[test]
    fn impulse_magnitude_with_linear_velocity_test() {
        let circle = RigidBodyBuilder::default().id(0)
            .body_type(RigidBodyType::Circle { radius: 5. })
            .mass(1.0)
            .velocity([10.,0.,0.])
            .position([-5.,400.,0.])
            .build();
        let rectangle = RigidBodyBuilder::default().id(1)
            .body_type(RigidBodyType::Rectangle { width: 10., height: 800. })
            .mass(1.0)
            .velocity([0.,0.,0.])
            .position([5.,0.,0.])
            .build();
        let collision_point = [0.0, 400.0,0.0];
        let collision_normal = [-1.0,0.0,0.0];
        let impulse = impulse_magnitude(1.0, &collision_normal, &collision_point, &circle, &rectangle);
        let post_angular_velocity_rect = post_collision_angular_velocity(
            &collision_normal, &collision_point, impulse, &rectangle);

        let impulse_ff: f32 = FixedFloat::from(impulse).into();
        let post_angular_velocity_rect_ff: f32 = FixedFloat::from(post_angular_velocity_rect).into();

        let _r_cp = [5.0, 0.0, 0.0];
        let _r_cp_perp = [0.0, 5.0, 0.0];
        let _r_rp = [-5.0, 400.0, 0.0];
        let _r_rp_perp = [-400.0, -5.0, 0.0];
        let _circle_inertia = 0.5*1.0*5.*5.;
        let _vel_diff = [10.0, 0.,0.];
        let _rectangle_inertia = (1.0*(10.*10. + 800.*800.))/12.0;
        let expected_impulse = 4.0;
        let expected_post_angular_velocity_rect = 0.03;

        assert_eq!(expected_impulse, impulse_ff, "Expected impulse {expected_impulse} but found {impulse_ff}");
        assert_eq!(expected_post_angular_velocity_rect, post_angular_velocity_rect_ff,
            "Expected post collision angular velocity for rectangle to be {expected_post_angular_velocity_rect} but found {post_angular_velocity_rect_ff}");
        
    }

    #[test]
    fn impulse_magnitude_with_angular_velocity_test() {
        let circle = RigidBodyBuilder::default().id(0)
            .position([-400.,-3.,0.])
            .mass(1.0)
            .velocity([0.,0.,0.])
            .body_type(RigidBodyType::Circle { radius: 5.0 })
            .build();
        let rectangle = RigidBodyBuilder::default().id(1)
            .position([0.,5.,0.])
            .mass(1.0)
            .velocity([0.,0.,0.])
            .rotational_velocity(std::f32::consts::PI/120.0)
            .body_type(RigidBodyType::Rectangle { width: 1000., height: 10.})
            .build();

        let collision_point = [-400.0, 0.0, 0.0];
        let collision_normal = [0.0,-1.0,0.0];
        let impulse = impulse_magnitude(1.0, &collision_normal, &collision_point, &circle, &rectangle);

        let post_velocity_circle = post_collision_velocity(&collision_normal, impulse, &circle);
        let post_velocity_rectangle = post_collision_velocity(&collision_normal, -impulse, &rectangle);

        let post_angular_velocity_circle = post_collision_angular_velocity(
            &collision_normal, &collision_point, impulse, &circle);
        let post_angular_velocity_rectangle = post_collision_angular_velocity(
            &collision_normal, &collision_point, -impulse, &rectangle);

        let impulse_ff: f32 = FixedFloat::from(impulse).into();
        let post_velocity_circle_ff: [f32; 3] = FixedFloatVector::from(post_velocity_circle).into();
        let post_velocity_rectangle_ff: [f32; 3] = FixedFloatVector::from(post_velocity_rectangle).into();
        let post_angular_velocity_circ_ff: f32 = FixedFloat::from(post_angular_velocity_circle).into();
        let post_angular_velocity_rect_ff: f32 = FixedFloat::from(post_angular_velocity_rectangle).into();

        let _r_cp = [0.0, 5.0, 0.0];
        let _r_cp_perp = [-5.0, 0.0, 0.0];
        let _r_rp = [-400.0, -5.0, 0.0];
        let _r_rp_perp = [5.0, -400.0, 0.0];
        let _circle_inertia = 0.5*1.0*5.*5.;
        let _vel_diff = [-5.0*std::f32::consts::PI/120., 400.0*std::f32::consts::PI/120., 0.0];
        let _rectangle_inertia = (1.0/12.0)*1.0*(1000.0_f32.powi(2) + 10.0_f32.powi(2));
        let expected_impulse = 5.343;
        let expected_post_velocity_circle = [0.0, -5.343, 0.0];
        let expected_post_velocity_rectangle = [0.0, 5.343, 0.0];
        let expected_post_angular_velocity_circle = 0.0;
        let expected_post_angular_velocity_rectangle = 0.001;

        assert_eq!(expected_impulse, impulse_ff, "Expected impulse {expected_impulse} but found {impulse_ff}");
        assert_eq!(expected_post_velocity_circle, post_velocity_circle_ff,
            "Expected post collision velocity for circle to be {expected_post_velocity_circle:?} but found {post_velocity_circle_ff:?}");
        assert_eq!(expected_post_velocity_rectangle, post_velocity_rectangle_ff,
            "Expected post collision velocity for rectangle to be {expected_post_velocity_rectangle:?} but found {post_velocity_rectangle_ff:?}");
        assert_eq!(expected_post_angular_velocity_circle, post_angular_velocity_circ_ff,
            "Expected post collision angular velocity for circle to be {expected_post_angular_velocity_circle} but found {post_angular_velocity_circ_ff}");
        assert_eq!(expected_post_angular_velocity_rectangle, post_angular_velocity_rect_ff,
            "Expected post collision angular velocity for rectangle to be {expected_post_angular_velocity_rectangle} but found {post_angular_velocity_rect_ff}");

    }      

    #[test]
    fn cross_2d_test() {
        let a = [-10., 15., 0.];
        let b = [-4.0, -2., 0.];
        assert_eq!(cross_2d(&a, &b), super::dot(&super::perpendicular_2d(&a), &b),
            "Expected cross_2d to give the same result as taking the dot product between a and b_perp");
    }
}
