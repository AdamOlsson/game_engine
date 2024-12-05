use crate::engine::physics_engine::collision::{RigidBody, RigidBodyType};

use cgmath::{InnerSpace, MetricSpace, Vector3};

/// Returns the moment of inertia for a solid circle rotating around its center
pub fn inertia(radius: f32, mass: f32) -> f32 {
    mass * radius.powi(2) / 2.0
}

/// Returns the left-, right-, top- and bottom-most points of a rotated rectangle
pub fn cardinals(center: [f32; 3], radius: f32) -> [[f32; 3]; 4] {
    let west = [center[0] - radius, center[1], 0.0];
    let east = [center[0] + radius, center[1], 0.0];
    let north = [center[0], center[1] + radius, 0.0];
    let south = [center[0], center[1] - radius, 0.0];

    return [west, east, north, south];
}

pub fn click_inside(point: (f32, f32), circle: &RigidBody) -> bool {
    let radius = match circle.body_type {
        RigidBodyType::Circle { radius } => radius,
        _ => unreachable!(),
    };
    let click_position = Vector3::new(point.0, point.1, 0.0);

    click_position.distance2(circle.position) < radius.powi(2)
}
