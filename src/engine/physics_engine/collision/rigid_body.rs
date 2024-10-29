use cgmath::Vector3;

use crate::engine::physics_engine::util::{circle_equations, equations, rectangle_equations};
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use crate::engine::util::color::blue;
use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;
use crate::engine::util::zero;

#[derive(Clone)]
pub enum RigidBodyType {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Unkown,
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
    //pub inertia: f32,

    // Render data
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

impl RigidBody {
    pub fn closest_point_on_rectangle(&self, other_point: Vector3<f32>) -> Vector3<f32> {
        let rectangle = &self;
        let (width, height) = match rectangle.body_type {
            RigidBodyType::Rectangle { width, height } => (width, height),
            _ => panic!("Self is not a rectangle"),
        };
        
        let transformed_other_point = other_point - rectangle.position;
        let local_circle_center = equations ::rotate_z(
            &transformed_other_point.into(), -rectangle.rotation);

        let local_closest_point_on_rect_x = (-width/2.0).max(local_circle_center[0].min(width/2.0));
        let local_closest_point_on_rect_y = (-height/2.0).max(local_circle_center[1].min(height/2.0));
        let local_closest_point_on_rect = Vector3::new(
                local_closest_point_on_rect_x, local_closest_point_on_rect_y, 0.0);
       
        let local_rotated_closest_point_on_rect_ = equations::rotate_z(
            &local_closest_point_on_rect.into(), rectangle.rotation);

        let local_rotated_closest_point_on_rect: Vector3<f32> = 
            FixedFloatVector::from(local_rotated_closest_point_on_rect_).into();

        let closest_point_on_rect = 
            local_rotated_closest_point_on_rect + rectangle.position;

        return closest_point_on_rect;
    }

    pub fn inertia(&self) -> f32 {
        match self.body_type {
            RigidBodyType::Rectangle { width, height } => 
                rectangle_equations::inertia(height, width, self.mass),
            RigidBodyType::Circle { radius } => 
                circle_equations::inertia(radius, self.mass),
            _ => panic!("Unkown body type"),
        }
    }

    pub fn cardinals(&self) -> [[f32;3];4]{
        match self.body_type {
            RigidBodyType::Rectangle { width, height } => 
                rectangle_equations::cardinals(&self.position.into(), width, height, self.rotation),
            RigidBodyType::Circle { radius } => 
                circle_equations::cardinals(self.position.into(), radius),
            _ => panic!("Unkown body type"),
        }
    }
}

impl std::fmt::Display for RigidBodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RigidBodyType::Circle { radius } => write!(f, "Circle({})", radius),
            RigidBodyType::Rectangle { width, height } => write!(f, "Rectangle({},{})", width, height),
            RigidBodyType::Unkown => write!(f, "Uknown"),
        }
    }
}

impl std::fmt::Display for RigidBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RigidBody{{ id: {}, position: ({},{},{}), velocity: ({},{},{}), type: {}: rotation: {}rad, angular velocity: {} }}",
            self.id,
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z,
            self.body_type, self.rotation, self.rotational_velocity,)
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
    //pub inertia: f32,
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
        let body_type = RigidBodyType::Unkown; 
        let mass = 1.0;
        let rotation = 0.0;
        //let inertia = 0.0;
        let color = blue();
        let sprite_coord = SpriteCoordinate::none();
        Self { velocity: velocity.into(), rotational_velocity,
            id, acceleration: acceleration.into(), prev_position,
            position: position.into(), body_type,mass,rotation,//inertia,
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

    pub fn rotational_velocity(mut self, radians: f32) -> Self {
        self.rotational_velocity = radians;
        self
    }
   
    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    //pub fn inertia(mut self, inertia: f32) -> Self {
    //    self.inertia = inertia;
    //    self
    //}

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
            color: self.color, sprite_coord: self.sprite_coord, //inertia: self.inertia,
            rotational_velocity: self.rotational_velocity,
        }
    }
}

#[cfg(test)]
mod rigid_body_tests {
    mod closest_point_on_rectangle {
        use super::super::{RigidBody, RigidBodyBuilder, RigidBodyType};
        use cgmath::Vector3;
        macro_rules! closest_point_on_rectangle_tests {
            ($($name:ident: $rectangle: expr, $other_point: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let r: RigidBody = $rectangle;
                        let expected_output: Vector3<f32> = $expected;
                        let output = r.closest_point_on_rectangle($other_point);
                        assert_eq!(expected_output, output, 
                            "Expected {expected_output:?} but found {output:?}");
                    }
                )*
            }
        }


        closest_point_on_rectangle_tests! {
            given_other_point_is_x_axis_aligned_when_no_rotation_expect_closest_point_to_be_axis_aligned:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(-15., 0., 0.), Vector3::new(-5., 0., 0.)

            given_other_point_is_x_axis_aligned_when_rect_rotated_90_degrees_expect_closest_point_to_be_axis_aligned:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .rotation(std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(-15., 0., 0.), Vector3::new(-5., 0., 0.)


            given_other_point_is_y_axis_aligned_when_no_rotation_expect_closest_point_to_be_axis_aligned:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(0., 15., 0.), Vector3::new(0., 5., 0.)


            given_other_point_is_y_axis_aligned_when_rect_rotated_90_degrees_expect_closest_point_to_be_axis_aligned:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .rotation(std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(0., 15., 0.), Vector3::new(0., 5., 0.)

            given_other_point_is_diagonally_when_no_rotation_expect_closest_point_to_be_corner:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(-15., 15., 0.), Vector3::new(-5., 5.0, 0.)
            
            given_other_point_is_diagonally_when_rect_rotated_neg_90_expect_closest_point_to_be_corner:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .rotation(-std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 10., height: 10.,})
                    .build(),
                Vector3::new(-15., 15., 0.), Vector3::new(-5., 5.0, 0.)

            given_other_point_is_diagonally_when_rect_rotated_45_degrees_expect_closest_point_to_be_on_diag:
                RigidBodyBuilder::default().id(0).position([0.,0.,0.])
                    .rotation(-std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle {width: 20., height: 20.,})
                    .build(),
                Vector3::new(-30., 30., 0.), Vector3::new(-7.071, 7.071, 0.)


            given_other_point_is_on_x_axis_when_rectangle_is_rotated_90_degrees_and_offset_on_y_axis_expect_closest_point_be_on_rect_edge:
                RigidBodyBuilder::default().id(0).position([0.,-150.,0.])
                    .rotation(std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 500., height: 500.,})
                    .build(),
                Vector3::new(-400., 0., 0.), Vector3::new(-250., 0., 0.)

            given_other_point_is_on_y_axis_when_rectangle_is_rotated_90_degrees_and_offset_on_y_axis_expect_closest_point_be_on_rect_edge:
                RigidBodyBuilder::default().id(0).position([0.,-150.,0.])
                    .rotation(-std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 500., height: 500.,})
                    .build(),
                Vector3::new(0., 400., 0.), Vector3::new(0., 100., 0.)

            given_other_point_is_on_x_axis_when_rectangle_is_rotated_90_degrees_and_offset_on_x_axis_expect_closest_point_be_on_rect_edge:
                RigidBodyBuilder::default().id(0).position([150.,0.,0.])
                    .rotation(-std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 500., height: 500.,})
                    .build(),
                Vector3::new(-400., 0., 0.), Vector3::new(-100., 0., 0.)

            given_other_point_is_on_y_axis_when_rectangle_is_rotated_90_degrees_and_offset_on_x_axis_expect_closest_point_be_on_rect_edge:
                RigidBodyBuilder::default().id(0).position([150.,0.,0.])
                    .rotation(-std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 500., height: 500.,})
                    .build(),
                Vector3::new(0., 400., 0.), Vector3::new(0., 250., 0.)
            
            given_point_is_async_offset_when_rectangle_is_rotated_90_degrees_and_async_offset_expect_closest_point_be_on_rect_edge:
                RigidBodyBuilder::default().id(0).position([-150.,50.,0.])
                    .rotation(std::f32::consts::PI/2.0)
                    .body_type(RigidBodyType::Rectangle {width: 200., height: 200.,})
                    .build(),
                Vector3::new(70., -25., 0.), Vector3::new(-50., -25., 0.)

        }
    }
}
