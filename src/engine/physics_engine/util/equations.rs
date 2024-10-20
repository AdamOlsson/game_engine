
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

pub fn impulse_magnitude(rel_vel_magnitude: f32 , mass_a: f32, mass_b: f32, c_r: f32) -> f32 {
    let nom = -(1.0+c_r)*rel_vel_magnitude;
    let denom = (1.0/mass_a) + (1.0/mass_b);
    nom / denom
}

pub fn rotate_z(v: &[f32; 3], theta: f32) -> [f32; 3] {
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    [
        v[0] * cos_theta - v[1] * sin_theta,
        v[0] * sin_theta + v[1] * cos_theta,
        v[2], // z remains unchanged
    ]
}



#[cfg(test)]
mod test {
    macro_rules! assert_slice_almost_eq {
        ($expected:expr, $output:expr, $epsilon:expr) => {{
            assert_eq!($expected.len(), $output.len(), "Expected length {} found length {}",
                $expected.len(), $output.len());
            $expected.iter().enumerate().for_each(|(i, e)| {
                let o = $output[i];
                assert!((e - o).abs() < $epsilon, 
                    "Expected {e} but found {o} at index {i}");
            })
        }};
    }

    macro_rules! rotate_z_tests {
        ($($name:ident: $expected: expr, $output: expr, $max_diff: expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_slice_almost_eq!($expected, $output, $max_diff) 
                }
            )*
        }
    }

    use super::rotate_z;
    use std::f32::consts::PI;

    rotate_z_tests! {
        given_x_unit_vector_direction_when_rotated_90_degrees_expect_y_unit_vector:
            [0.0,1.0,0.0], rotate_z(&[1.0,0.0,0.0], PI/2.0), 1e-6
        given_y_unit_vector_direction_when_rotated_90_degrees_expect_negative_x_unit_vector:
            [-1.0,0.0,0.0], rotate_z(&[0.0,1.0,0.0], PI/2.0), 1e-6
        given_negative_x_unit_vector_direction_when_rotated_90_degrees_expect_negative_y_unit_vector:
            [0.0,-1.0,0.0], rotate_z(&[-1.0,0.0,0.0], PI/2.0), 1e-6
        given_negative_y_unit_vector_direction_when_rotated_90_degrees_expect_x_unit_vector:
            [1.0,0.0,0.0], rotate_z(&[0.0,-1.0,0.0], PI/2.0), 1e-6
        given_x_unit_vector_when_rotated_45_degrees_expect_vector_rotated:
            [0.707106,0.707106,0.0], rotate_z(&[1.0,0.0,0.0], PI/4.0), 1e-6

    }
}