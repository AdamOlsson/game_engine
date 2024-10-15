use super::{physics_engine::collision::collision_body::{CollisionBody, CollisionBodyType}, renderer_engine::shapes::{circle::CircleInstance, rectangle::RectangleInstance}};

pub mod log_performance;
pub mod color;

pub fn zero() -> [f32; 3] { 
    [0.,0.,0.] 
}


pub fn get_circle_instances(bodies: &[CollisionBody]) -> Vec<CircleInstance> {
    bodies.iter().filter_map(
        |body| {
            match body.body_type { 
                CollisionBodyType::Circle { radius } => 
                    Some(CircleInstance {
                        position: body.position.into(), 
                        color: body.color.into(), 
                        radius,
                        sprite_coord: body.sprite_coord.coordinate, 
                    }),
                _ => None
            }
        }).collect::<Vec<_>>()
}

pub fn get_rectangle_instances(bodies: &[CollisionBody]) -> Vec<RectangleInstance> {
    bodies.iter().filter_map(
        |body| {
            match body.body_type { 
                CollisionBodyType::Rectangle{ width, height } => 
                    Some(RectangleInstance{
                        color: body.color.into(), 
                        position: body.position.into(),
                        width,height,
                        sprite_coord: body.sprite_coord.coordinate,
                    }),
                _ => None
            }
        }).collect::<Vec<_>>()
}

