

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

