use cgmath::Vector3;

use crate::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;

#[derive(Clone)]
pub enum CollisionBodyType {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

#[derive(Clone)]
pub struct CollisionBody {
    pub id: usize,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub prev_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub body_type: CollisionBodyType,

    // Render data
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

impl CollisionBody {

    pub fn circle(
        id: usize, velocity: [f32;3], acceleration: [f32;3], position: [f32; 3],
        color: [f32;3], radius: f32 
    ) -> Self {
        let body_type = CollisionBodyType::Circle { radius };
        Self::new(id, velocity, acceleration, position, body_type, color)
    }

    pub fn rectangle(
        id: usize, velocity: [f32;3], acceleration: [f32;3], position: [f32; 3],
        color: [f32;3], width: f32, height: f32,
    ) -> Self {
        let body_type = CollisionBodyType::Rectangle { width, height };
        Self::new(id, velocity, acceleration, position, body_type, color)
    }

    fn new(
        id: usize, velocity: [f32;3], acceleration: [f32;3],
        position: [f32;3], body_type: CollisionBodyType,
        color: [f32;3]
    ) -> CollisionBody {
        let velocity = Vector3::from(velocity);
        let position = Vector3::from(position);
        let prev_position = position - velocity;
        let acceleration = Vector3::from(acceleration);
        let color = Vector3::from(color);
        let sprite_coord = SpriteCoordinate::none();
        Self {id, velocity, position, acceleration, color, body_type,
            prev_position, sprite_coord,
        }


    }

    pub fn set_sprite(&mut self, coord: SpriteCoordinate) {
        self.sprite_coord = coord;
    }
 }

impl std::fmt::Display for CollisionBodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollisionBodyType::Circle { radius } => write!(f, "Circle({})", radius),
            CollisionBodyType::Rectangle { width, height } => write!(f, "Rectangle({},{})", width, height),
        }
    }
}

impl std::fmt::Display for CollisionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CollisionBody{{ id: {}, position: ({},{},{}), prev_pos: ({},{},{}), velocity: ({},{},{}), type: {} }}",
            self.id,
            self.position.x, self.position.y, self.position.z,
            self.prev_position.x, self.prev_position.y, self.prev_position.z,
            self.velocity.x, self.velocity.y, self.velocity.z,
            self.body_type,)
    }
}


