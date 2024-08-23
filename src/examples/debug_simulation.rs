use cgmath::{ Vector3, Zero};
use crate::engine::physics_engine::collision::collision_body::CollisionBody;
use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use crate::engine::physics_engine::narrowphase::naive::Naive;
use crate::engine::renderer_engine::shapes::Shape;
use crate::engine::Simulation;
use crate::engine::renderer_engine::shapes::circle::Circle;
use crate::engine::physics_engine::narrowphase::NarrowPhase;
use crate::engine::physics_engine::integrator::verlet::VerletIntegrator;
use crate::engine::physics_engine::constraint::Constraint;
use crate::engine::physics_engine::constraint::resolver::elastic::ElasticConstraintResolver;
use crate::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use crate::engine::physics_engine::broadphase::BroadPhase;
use crate::engine::physics_engine::broadphase::blockmap::BlockMap;
use crate::engine::init_utils::create_grid_positions;

pub struct DebugSimulation {
    dt: f32,
    num_instances: u32,
    integrator: VerletIntegrator,
    constraint: Box<dyn Constraint>,
    broadphase: Box<dyn BroadPhase>,
    narrowphase: Box<dyn NarrowPhase>,
}

impl DebugSimulation {
    pub fn new(window_size: &winit::dpi::PhysicalSize<u32>) -> Self {
        let dt = 0.001;
        
        let velocities = vec![Vector3::new(-5., 0.5, 0.0),
                              Vector3::new(5., 0., 0.0),
                              Vector3::new(0.1, 5., 0.0),];
        let prev_positions = create_grid_positions(3, 1, 400.0, None);
        let position = vec![prev_positions[0] + velocities[0],
                            prev_positions[1] + velocities[1],
                            prev_positions[2] + velocities[2]];
        let colors = vec![
            Vector3::new(255.0,0.0,0.0),
            Vector3::new(0.0,255.0,0.0),
            Vector3::new(0.0,0.0,255.0),
            Vector3::new(0.0,0.0,255.0),
        ];
        let radius = vec![100.0, 100.0, 120.0];
        let bodies = vec![
            CollisionBody::circle(0, Vector3::zero(), Vector3::zero(),prev_positions[0], position[0], radius[0], colors[0]),
            CollisionBody::circle(1, Vector3::zero(), Vector3::zero(),prev_positions[1], position[1], radius[1], colors[1]),
            CollisionBody::circle(2, Vector3::zero(), Vector3::zero(),prev_positions[2], position[2], radius[2], colors[2]),
            CollisionBody::rectangle(3, Vector3::zero(),Vector3::zero(), Vector3::zero(), Vector3::zero(), 100., 100., colors[3]),
        ];
        let num_instances = bodies.len() as u32;
        let integrator = VerletIntegrator::new(f32::MAX, bodies);
        
        let mut constraint = Box::new(BoxConstraint::new(ElasticConstraintResolver::new()));
        constraint.set_top_left(Vector3::new(-(window_size.width as f32), window_size.height as f32, 0.0));
        constraint.set_bottom_right(Vector3::new(window_size.width as f32, -(window_size.height as f32), 0.0));
        let broadphase = Box::new(BlockMap::new(window_size.width as f32));
        let narrowphase = Box::new(Naive::new(SimpleCollisionSolver::new()));

            
        Self { 
            dt, integrator, constraint, broadphase, narrowphase,
            num_instances}
    }
}

impl Simulation for DebugSimulation {
    fn update(&mut self) {
        self.integrator.update(self.dt);
        let bodies = self.integrator.get_bodies_mut();

        for b in bodies.iter_mut() {
            self.constraint.apply_constraint(b);
        }

        let candidates = self.broadphase.collision_detection(bodies);
        for c in candidates.iter() {
            self.narrowphase.collision_detection(bodies, c);
        }

        //bodies.iter().for_each(|b| println!("{}", b));
    }

    fn get_bodies(&self) -> &Vec<CollisionBody> {
        &self.integrator.get_bodies()
    }

    fn get_num_active_instances(&self) -> u32 {
        self.num_instances
    }

    fn get_target_num_instances(&self) -> u32 {
        self.num_instances
    }
}
