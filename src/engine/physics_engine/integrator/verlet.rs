use cgmath::InnerSpace;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use crate::engine::physics_engine::collision::rigid_body::RigidBody;

pub struct VerletIntegrator {
    velocity_cap: f32,
}

impl VerletIntegrator {
    pub fn new(velocity_cap: f32) -> Self {
        Self { velocity_cap }
    }

    pub fn update(&self, bodies: &mut Vec<RigidBody>, dt: f32) {
        bodies.par_iter_mut()
            .for_each(|b|{
                // Transform vecocity (uses verlet and linear)
                let mut velocity = b.position - b.prev_position;
                let vel_magn = velocity.magnitude();
                if vel_magn > self.velocity_cap {
                    velocity = velocity*(self.velocity_cap/vel_magn)
                }
                b.prev_position = b.position;
                b.position = b.position + velocity + b.acceleration * dt*dt;   
                b.velocity = velocity; // Used in constraint handling

                // Rotational (Euler based and non-linear)
                b.rotation += b.rotational_velocity*dt;
            });
    }

    pub fn set_velocity_x(&self, bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        let p = bodies[idx].position.x;
        bodies[idx].prev_position.x = p - new;  
    }

    pub fn set_velocity_y(&mut self, bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        let p = bodies[idx].position.y;
        bodies[idx].prev_position.y = p - new;  
    }
    
    pub fn set_acceleration_x(&self, bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].acceleration.x = new;
    }
    
    pub fn set_acceleration_y(&self, bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].acceleration.y = new;
    }

    pub fn set_position_x(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].position.x = new;
        bodies[idx].prev_position.x = new - bodies[idx].velocity.x;
    }
    
    pub fn set_position_y(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].position.y = new;
        bodies[idx].prev_position.y = new - bodies[idx].velocity.y;
    }

    pub fn set_rotation(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].rotation = new;
    }

    pub fn set_rotational_velocity(bodies: &mut Vec<RigidBody>, idx: usize, new: f32) {
        bodies[idx].rotational_velocity = new;
    }
}
