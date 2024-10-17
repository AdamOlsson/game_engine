use cgmath::InnerSpace;
use crate::engine::physics_engine::collision::rigid_body::RigidBody;

pub struct VerletIntegrator {
    velocity_cap: f32,
    bodies: Vec<RigidBody>,
}

impl VerletIntegrator {
    pub fn new(
        velocity_cap: f32, bodies: Vec<RigidBody>
    ) -> Self {
        Self { velocity_cap, bodies}
    }

    pub fn update(&mut self, dt: f32) {
        for b in self.bodies.iter_mut() {
            let mut velocity = b.position - b.prev_position;
            let vel_magn = velocity.magnitude();
            if vel_magn > self.velocity_cap {
                velocity = velocity*(self.velocity_cap/vel_magn)
            }
            b.prev_position = b.position;
            b.position = b.position + velocity + b.acceleration * dt*dt;   
            b.velocity = velocity; // Used in constraint handling
        }
    }

    pub fn set_velocity_x(&mut self, idx: usize, new: f32) {
        let p = self.bodies[idx].position.x;
        self.bodies[idx].prev_position.x = p - new;  
    }

    pub fn set_velocity_y(&mut self, idx: usize, new: f32) {
        let p = self.bodies[idx].position.y;
        self.bodies[idx].prev_position.y = p - new;  
    }
    
    pub fn set_acceleration_x(&mut self, idx: usize, new: f32) {
        self.bodies[idx].acceleration.x = new;
    }
    
    pub fn set_acceleration_y(&mut self, idx: usize, new: f32) {
        self.bodies[idx].acceleration.y = new;
    }

    pub fn set_position_x(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].position.x = new;
        bodies[idx].prev_position.x = new - bodies[idx].velocity.x;
    }
    
    pub fn set_position_y(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].position.y = new;
        bodies[idx].prev_position.y = new - bodies[idx].velocity.y;
    }

    pub fn get_bodies(&self) -> &Vec<RigidBody> {
        &self.bodies
    }

    pub fn get_bodies_mut(&mut self) -> &mut Vec<RigidBody> {
        &mut self.bodies
    }

    #[allow(dead_code)]
    pub fn update_subset(&mut self){
        todo!("Not yet implementd");
    }
}
