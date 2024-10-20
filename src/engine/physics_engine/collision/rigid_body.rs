use cgmath::Vector3;

use crate::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use crate::engine::util::color::blue;
use crate::engine::util::fixed_float::fixed_float::FixedFloat;
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
    pub rotation: FixedFloat,

    pub rotational_velocity: FixedFloat, 
    pub inertia: f32,

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
        
        // TODO: Input args should eventually be FixedFloatVector
        let width_ff = FixedFloat::from(width);
        let height_ff = FixedFloat::from(height);
        let rectangle_pos_ff = FixedFloatVector::from(rectangle.position);
        let other_point_ff = FixedFloatVector::from(other_point);
        let rect_rotation_ff = FixedFloat::from(rectangle.rotation);

        let local_circle_center_ff = (&other_point_ff - &rectangle_pos_ff).rotate_z(&-rect_rotation_ff);

        let local_closest_point_on_rect_x = (-width_ff/2.0).max(&local_circle_center_ff.x.min(&(width_ff/2.0)));
        let local_closest_point_on_rect_y = (-height_ff/2.0).max(&local_circle_center_ff.y.min(&(height_ff/2.0)));
        let local_closest_point_on_rect = FixedFloatVector::new(
            local_closest_point_on_rect_x, local_closest_point_on_rect_y, FixedFloat::from(0.0));
        
        let transformed_local_closest_point = local_closest_point_on_rect + rectangle_pos_ff;
        let closest_point_on_rect = transformed_local_closest_point.rotate_z(&rect_rotation_ff);

        return closest_point_on_rect.into();
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
            "RigidBody{{ id: {}, position: ({},{},{}), prev_pos: ({},{},{}), velocity: ({},{},{}), type: {}: rotation: {}rad }}",
            self.id,
            self.position.x, self.position.y, self.position.z,
            self.prev_position.x, self.prev_position.y, self.prev_position.z,
            self.velocity.x, self.velocity.y, self.velocity.z,
            self.body_type, self.rotation,)
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
    pub rotational_velocity: FixedFloat, 
    pub rotation: FixedFloat,
    pub inertia: f32,
    // Render data
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

impl std::default::Default for RigidBodyBuilder {
    fn default() -> Self {
        let id = None;
        let velocity = zero();
        let rotational_velocity = FixedFloat::from(0.0);
        let acceleration = zero();
        let prev_position = None;
        let position = zero();
        let body_type = RigidBodyType::Unkown; 
        let mass = 1.0;
        let rotation = FixedFloat::from(0.0);
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

    pub fn rotational_velocity(mut self, radians: f32) -> Self {
        self.rotational_velocity = FixedFloat::from(radians);
        self
    }
   
    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = FixedFloat::from(rotation);
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

        }
    }
}
