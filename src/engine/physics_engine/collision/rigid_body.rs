use cgmath::Vector3;

use crate::engine::{renderer_engine::asset::sprite_sheet::SpriteCoordinate, util::{color::blue, zero}};

#[derive(Clone)]
pub enum RigidBodyType {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

#[derive(Clone)]
pub struct RigidBody {
    pub id: usize,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub prev_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub body_type: RigidBodyType,
    pub mass: f32,
    pub rotation: f32,

    pub rotational_velocity: f32, 
    pub inertia: f32,

    // Render data
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

impl RigidBody {}

impl std::fmt::Display for RigidBodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RigidBodyType::Circle { radius } => write!(f, "Circle({})", radius),
            RigidBodyType::Rectangle { width, height } => write!(f, "Rectangle({},{})", width, height),
        }
    }
}

impl std::fmt::Display for RigidBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RigidBody{{ id: {}, position: ({},{},{}), prev_pos: ({},{},{}), velocity: ({},{},{}), type: {} }}",
            self.id,
            self.position.x, self.position.y, self.position.z,
            self.prev_position.x, self.prev_position.y, self.prev_position.z,
            self.velocity.x, self.velocity.y, self.velocity.z,
            self.body_type,)
    }
}

pub struct RigidBodyBuilder {
    pub id: Option<usize>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub prev_position: Option<Vector3<f32>>,
    pub position: Vector3<f32>,
    pub body_type: RigidBodyType,
    pub mass: f32,
    pub rotational_velocity: f32, 
    pub rotation: f32,
    pub inertia: f32,
    // Render data
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

impl std::default::Default for RigidBodyBuilder {
    fn default() -> Self {
        let id = None;
        let velocity = zero();
        let rotational_velocity = 0.0; 
        let acceleration = zero();
        let prev_position = None;
        let position = zero();
        let body_type = RigidBodyType::Circle { radius: 80.0 };
        let mass = 1.0;
        let rotation = 0.0;
        let inertia = 0.0;
        let color = blue();
        let sprite_coord = SpriteCoordinate::none();
        Self { velocity: velocity.into(), rotational_velocity,
            id, acceleration: acceleration.into(), prev_position,
            position: position.into(), body_type,mass,rotation,inertia,
            color: color.into(),sprite_coord,
        }
    }
}

impl RigidBodyBuilder {
   
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    pub fn velocity(mut self, velocity: [f32;3]) -> Self {
        self.velocity = velocity.into();
        self
    }

    pub fn acceleration(mut self, acceleration: [f32;3]) -> Self {
        self.acceleration = acceleration.into();
        self
     }

    pub fn prev_position(mut self, prev_position: [f32;3]) -> Self {
        self.prev_position = Some(prev_position.into());
        self
    }

    pub fn position(mut self, position: [f32;3]) -> Self {
        self.position = position.into();
        self
    }

    pub fn body_type(mut self, body_type: RigidBodyType) -> Self {
        self.body_type = body_type;
        self
    } 

    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn rotational_velocity(mut self, rotational_velocity: f32) -> Self {
        self.rotational_velocity = rotational_velocity;
        self
    }
   
    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn inertia(mut self, inertia: f32) -> Self {
        self.inertia = inertia;
        self
    }

    pub fn color(mut self, color: [f32;3]) -> Self {
        self.color = color.into();
        self
    }

    pub fn sprite_coord(mut self, sprite_coord: SpriteCoordinate) -> Self {
        self.sprite_coord = sprite_coord;
        self
    }
 
    pub fn build(self) -> RigidBody {
        let id = match self.id {
            Some(id) => id,
            None => panic!("RigidBody id needs to be set"),
        };

        let prev_position = match self.prev_position {
            Some(pp) => pp,
            None => self.position - self.velocity,
        };

        RigidBody { id, velocity: self.velocity, acceleration: self.acceleration, 
            prev_position, position: self.position,
            body_type: self.body_type, mass: self.mass, rotation: self.rotation,
            color: self.color, sprite_coord: self.sprite_coord, inertia: self.inertia,
            rotational_velocity: self.rotational_velocity,
        }
    }
}
