use cgmath::Vector3;
use crate::engine::physics_engine::collision::rigid_body::{RigidBody, RigidBodyType};
use crate::engine::physics_engine::util::{circle_equations, rectangle_equations};
use super::{resolver::ConstraintResolver, Constraint};


pub struct BoxConstraint {
    resolver: Box<dyn ConstraintResolver>,
    top_left: Vector3<f32>,
    bottom_right: Vector3<f32>
}

impl BoxConstraint {
    pub fn new<T: ConstraintResolver + 'static>(resolver: T) -> Self {
        let top_left = Vector3::new(-1.0, 1.0, 0.0);
        let bottom_right = Vector3::new(1.0, -1.0, 0.0);
        Self {resolver: Box::new(resolver), top_left, bottom_right }
    }

    #[allow(dead_code)] 
    pub fn set_top_left(&mut self, top_left: Vector3<f32>) {
        self.top_left = top_left;
    }
    
    #[allow(dead_code)] 
    pub fn set_bottom_right(&mut self, bottom_right: Vector3<f32>) {
        self.bottom_right = bottom_right;
    }
}

impl Constraint for BoxConstraint {
    fn apply_constraint(&self, body: &mut RigidBody) {
        let cardinals = match body.body_type {
            RigidBodyType::Circle { radius } => 
                circle_equations::cardinals(body.position.into(), radius),
            RigidBodyType::Rectangle { width, height } =>
                rectangle_equations::cardinals(body.position.into(), width, height, body.rotation),
            _ => panic!("Invalid body type {}", body.body_type),
        };

        let left_most = cardinals[0];
        let right_most = cardinals[1];
        let top_most = cardinals[2];
        let bot_most = cardinals[3];

        // Left side
        if left_most[0] < self.top_left.x {
            let diff = left_most[0] - self.top_left.x;
            self.resolver.resolve_horizontal(diff, body);
        }
        // Right side
        if right_most[0] > self.bottom_right.x {
            let diff = right_most[0] - self.bottom_right.x; 
            self.resolver.resolve_horizontal(diff, body);
        }
        // Bottom side
        if bot_most[1] < self.bottom_right.y {
            let diff = bot_most[1] - self.bottom_right.y;
            self.resolver.resolve_vertical(diff, body);
        }
        // Top side
        if top_most[1] > self.top_left.y {
            let diff = top_most[1] - self.top_left.y;
            self.resolver.resolve_vertical(diff, body);
        }
    }
}
