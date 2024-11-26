use super::{
    entity::EntityHandle,
    physics_engine::collision::RigidBodyType,
    renderer_engine::shapes::{circle::CircleInstance, rectangle::RectangleInstance},
};

pub mod color;
pub mod fixed_float;
pub mod log_performance;

pub fn zero() -> [f32; 3] {
    [0., 0., 0.]
}

pub fn get_circle_instances(entities: &[EntityHandle]) -> Vec<CircleInstance> {
    entities
        .iter()
        .filter_map(|entity| match entity.rigid_body.unwrap().body_type {
            RigidBodyType::Circle { radius } => Some(CircleInstance {
                position: entity.rigid_body.unwrap().position.into(),
                color: entity.render_body.unwrap().color.into(),
                rotation: entity.rigid_body.unwrap().rotation,
                radius,
                sprite_coord: entity.render_body.unwrap().sprite_coord.coordinate,
            }),
            _ => None,
        })
        .collect::<Vec<_>>()
}

pub fn get_rectangle_instances(entities: &[EntityHandle]) -> Vec<RectangleInstance> {
    entities
        .iter()
        .filter_map(|entity| match entity.rigid_body.unwrap().body_type {
            RigidBodyType::Rectangle { width, height } => Some(RectangleInstance {
                color: entity.render_body.unwrap().color.into(),
                rotation: entity.rigid_body.unwrap().rotation.into(),
                position: entity.rigid_body.unwrap().position.into(),
                width,
                height,
                sprite_coord: entity.render_body.unwrap().sprite_coord.coordinate,
            }),
            _ => None,
        })
        .collect::<Vec<_>>()
}
