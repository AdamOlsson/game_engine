use cgmath::Vector3;

pub fn red() -> Vector3<f32> {
    Vector3::new(255.0,0.0,0.0)
}

pub fn green() -> Vector3<f32> {
    Vector3::new(0.0,255.0,0.0)
}

pub fn blue() -> Vector3<f32> {
    Vector3::new(0.0,0.0,255.0)
}
