use cgmath::Vector3;

#[derive(Clone)]
pub enum CollisionBodyType {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

#[derive(Clone)]
pub struct CollisionBody {
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub prev_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub id: usize,

    pub body_type: CollisionBodyType
}

impl CollisionBody {

    pub fn circle(
        id: usize, velocity: Vector3<f32>, acceleration: Vector3<f32>,
        prev_position: Vector3<f32>, position: Vector3<f32>, radius: f32
    ) -> Self {
        Self {
            velocity,
            acceleration,
            prev_position,
            position,
            id,
            body_type: CollisionBodyType::Circle { radius }
        }
    }

    pub fn rectangle(
        id: usize, velocity: Vector3<f32>, acceleration: Vector3<f32>,
        prev_position: Vector3<f32>, position: Vector3<f32>, width: f32, height: f32
    ) -> Self {
        Self {
            velocity,
            acceleration,
            prev_position,
            position,
            id,
            body_type: CollisionBodyType::Rectangle { width, height }
        }
    }
 }

impl std::fmt::Display for CollisionBodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollisionBodyType::Circle { radius } => write!(f, "Circle({})", radius),
            CollisionBodyType::Rectangle { width, height } => write!(f, "Rectangle({},{}", width, height),
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
            self.body_type,
            self.velocity.x, self.velocity.y, self.velocity.z)
    }
}


