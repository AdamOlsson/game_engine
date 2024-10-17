use crate::engine::physics_engine::{collision::rigid_body::RigidBody, util::equations::inelastic_collision_1d};

use super::ConstraintResolver;


pub struct InelasticConstraintResolver {}
impl InelasticConstraintResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConstraintResolver for InelasticConstraintResolver {
    fn resolve_vertical(&self, diff: f32, body: &mut RigidBody) {
        let mass_body = 1.0;
        let vel_y_body = body.velocity.y;

        let c_r = 1.0;
        let mass_wall: f32 = 1_000_000_000.0;
        let vel_y_wall: f32 = 0.0;
        let (_,new_vel_y_body) = inelastic_collision_1d(
            mass_wall, mass_body, vel_y_wall, vel_y_body, c_r);
        body.position.y -= diff;
        body.velocity.y = new_vel_y_body;
        body.prev_position = body.position - body.velocity;
    }

    fn resolve_horizontal(&self, diff: f32, body: &mut RigidBody) {
        let mass_body = 1.0;
        let vel_x_body = body.velocity.x;

        let c_r = 1.0;
        let mass_wall: f32 = 1_000_000_000.0;
        let vel_x_wall: f32 = 0.0;
        let (_,new_vel_x_body) = inelastic_collision_1d(
            mass_wall, mass_body, vel_x_wall, vel_x_body, c_r);
        body.position.x -= diff;
        body.velocity.x = new_vel_x_body;
        body.prev_position = body.position - body.velocity;
    }
}
