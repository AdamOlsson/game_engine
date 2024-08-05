use cgmath::InnerSpace;
use crate::engine::physics_engine::collision::collision_body::CollisionBody;

pub struct VerletIntegrator {
    velocity_cap: f32,
    bodies: Vec<CollisionBody>,
}

impl VerletIntegrator {
    pub fn new(
        velocity_cap: f32, bodies: Vec<CollisionBody>
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
            //self.prev_positions[i] = b.position;
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

    pub fn get_bodies(&self) -> &Vec<CollisionBody> {
        &self.bodies
    }

    pub fn get_bodies_mut(&mut self) -> &mut Vec<CollisionBody> {
        &mut self.bodies
    }

    #[allow(dead_code)]
    pub fn update_subset(&mut self){
        todo!("Not yet implementd");
    }
}
