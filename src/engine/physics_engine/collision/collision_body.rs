use cgmath::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct CollisionBody {
    pub velocity: Vector3<f32>,
    pub prev_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub radius: f32,
    pub id: usize
}

impl CollisionBody {
    pub fn new(
        id: usize, velocity: Vector3<f32>, prev_position: Vector3<f32>,
        position: Vector3<f32>, radius: f32
    ) -> Self {
        Self {
            velocity,
            prev_position,
            position,
            radius,
            id
        }
    }
}

impl std::fmt::Display for CollisionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, position: ({},{},{}), prev_pos: ({},{},{}), radius: {}, velocity: ({},{},{})",
            self.id,
            self.position.x, self.position.y, self.position.z,
            self.prev_position.x, self.prev_position.y, self.prev_position.z,
            self.radius,
            self.velocity.x, self.velocity.y, self.velocity.z)
    }
}


