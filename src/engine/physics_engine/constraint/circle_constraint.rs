use core::panic;


use cgmath::InnerSpace;

use crate::engine::physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType};

use super::Constraint;


pub struct CircleConstraint {
    radius: f32,
}

impl CircleConstraint{
    pub fn new(radius: f32) -> Self {
        Self {radius}
    }
}

impl Constraint for CircleConstraint {
    fn apply_constraint(&self, body: &mut CollisionBody) {
        let object_radius = match body.body_type {
            CollisionBodyType::Circle { radius } => radius,
            _ => panic!("Cirlce constraint only supports circle shaped bodies for now"),
        };

        let constraint_radius = self.radius;  
        let dist_to_center = body.position.magnitude(); 
        if dist_to_center + object_radius > constraint_radius {
            let excess_dist = dist_to_center + object_radius - constraint_radius;
            let correction_direction = body.position.normalize();
            body.position = correction_direction * (dist_to_center - excess_dist);
        }
    }
}
