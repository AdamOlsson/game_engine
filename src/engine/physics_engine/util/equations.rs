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

pub fn impulse_magnitude(
    e: f32, coll_normal: &[f32;3], collision_point: &[f32;3],
    body_a: &RigidBody, body_b: &RigidBody,
) -> f32 {
    let rel_vel = body_a.velocity - body_b.velocity;
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

    use super::rotate_z;
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
}
