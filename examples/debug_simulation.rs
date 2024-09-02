extern crate game_engine;

use cgmath::{ Vector3, Zero};
use game_engine::engine::game_engine::GameEngineBuilder;
use winit::dpi::PhysicalSize;

use game_engine::engine::physics_engine::collision::collision_body::CollisionBody;
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::Simulation;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::constraint::resolver::elastic::ElasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::broadphase::blockmap::BlockMap;
use game_engine::engine::init_utils::create_grid_positions;

pub struct DebugSimulation {
    dt: f32,
    integrator: VerletIntegrator,
    constraint: Box<dyn Constraint>,
    broadphase: Box<dyn BroadPhase>,
    narrowphase: Box<dyn NarrowPhase>,
}

const SPRITE_WIDTH: f32 = 16.;
const SPRITE_HEIGHT: f32 = 16.;
const SPRITE_SHEET_WIDTH: f32 = 128.;
const SPRITE_SHEET_HEIGHT: f32 = 128.;

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
        let mut bodies = vec![
            CollisionBody::circle(0, Vector3::zero(), Vector3::zero(),prev_positions[0], position[0], radius[0], colors[0]),
            CollisionBody::circle(1, Vector3::zero(), Vector3::zero(),prev_positions[1], position[1], radius[1], colors[1]),
            CollisionBody::circle(2, Vector3::zero(), Vector3::zero(),prev_positions[2], position[2], radius[2], colors[2]),
            CollisionBody::rectangle(3, Vector3::zero(),Vector3::zero(), Vector3::zero(), Vector3::zero(), 200., 200., colors[3]),
        ];
         
        bodies[3].set_texture_cell(2); // u32::MAX == No sprite

        let integrator = VerletIntegrator::new(f32::MAX, bodies);
        
        let mut constraint = Box::new(BoxConstraint::new(ElasticConstraintResolver::new()));
        constraint.set_top_left(Vector3::new(-(window_size.width as f32), window_size.height as f32, 0.0));
        constraint.set_bottom_right(Vector3::new(window_size.width as f32, -(window_size.height as f32), 0.0));
        let broadphase = Box::new(BlockMap::new(window_size.width as f32));
        let narrowphase = Box::new(Naive::new(SimpleCollisionSolver::new()));

            
        Self { 
            dt, integrator, constraint, broadphase, narrowphase, }
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
        let graphs: Vec<CollisionGraph> = candidates.iter()
            .map(|c| self.narrowphase.collision_detection(bodies, c))
            .collect();

        let rect_id = 3;
        bodies[rect_id].color = Vector3::new(0.0,255.0,255.0);
        for g in graphs {
            for pairs in g.collisions {
                if pairs.0 == rect_id || pairs.1 == rect_id {
                    bodies[rect_id].color = Vector3::new(255.0,255.0,0.0);
                } 
            }
        }
    }

    fn get_bodies(&self) -> &Vec<CollisionBody> {
        &self.integrator.get_bodies()
    }
}

fn main() {
    let window_size = PhysicalSize::new(1000, 800);
    let simulation = DebugSimulation::new(&window_size);
    
    let texture_sprite_sheet_bytes = include_bytes!("../assets/sprite_sheet.png");
    let texture_sprite_sheet_buf = image::load_from_memory(texture_sprite_sheet_bytes).unwrap();
    let texture_sprite_sheet_rgb = texture_sprite_sheet_buf.to_rgba8();

    let engine = GameEngineBuilder::new()
        .physics_engine(simulation)
        .window_size(window_size)
        .texture(texture_sprite_sheet_rgb)
        .build();

    engine.run();
}
