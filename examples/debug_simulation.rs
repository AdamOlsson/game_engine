extern crate game_engine;

use cgmath::{ Vector3, Zero};
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::renderer_engine::asset::asset::Asset;
use game_engine::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use game_engine::engine::renderer_engine::render_engine::RenderEngineControl;
use winit::dpi::PhysicalSize;

use game_engine::engine::physics_engine::collision::collision_body::CollisionBody;
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::{PhysicsEngine, RenderEngine};
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::constraint::resolver::elastic::ElasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::broadphase::blockmap::BlockMap;
use game_engine::engine::init_utils::create_grid_positions;

pub struct DebugPhysicsEngine {
    dt: f32,
    integrator: VerletIntegrator,
    constraint: Box<dyn Constraint>,
    broadphase: Box<dyn BroadPhase>,
    narrowphase: Box<dyn NarrowPhase>,
}

impl DebugPhysicsEngine {
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
            CollisionBody::circle(0, Vector3::zero(), Vector3::zero(), prev_positions[0], position[0], radius[0], colors[0]),
            CollisionBody::circle(1, Vector3::zero(), Vector3::zero(), prev_positions[1], position[1], radius[1], colors[1]),
            CollisionBody::circle(2, Vector3::zero(), Vector3::zero(), prev_positions[2], position[2], radius[2], colors[2]),
            CollisionBody::rectangle(3, Vector3::zero(),Vector3::zero(), Vector3::zero(), Vector3::zero(), 200., 200., colors[3]),
            //CollisionBody::rectangle(4, Vector3::zero(),Vector3::zero(), Vector3::zero(), Vector3::zero(), 400., 200., colors[3]),
        ];
        

        bodies[0].set_sprite(SpriteCoordinate::new([2.,0.], [3.,1.]));
        bodies[3].set_sprite(SpriteCoordinate::new([1.,0.], [2.,1.]));
        
        //bodies[4].prev_position = Vector3::new(-500., -200., 0.0);
        //bodies[4].position = Vector3::new(-500., -200., 0.0);
        //bodies[4].set_sprite(SpriteCoordinate::new([0.,0.], [2.,1.]));

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

impl RenderEngine for DebugPhysicsEngine {
    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        //let rect_instances = GameEngine::get_rectangle_instances(&self.physics_engine);
        //let circle_instances = GameEngine::get_circle_instances(&self.physics_engine);

    }
}

impl PhysicsEngine for DebugPhysicsEngine {
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
    let physics_engine = DebugPhysicsEngine::new(&window_size);
    
    let sprite_sheet_bytes = include_bytes!("../assets/sprite_sheet.png");
    let sprite_sheet_asset  = Asset::sprite_sheet(sprite_sheet_bytes, 16, 16);
    
    let background_bytes = include_bytes!("../assets/background.png"); // TODO
    let background_asset = Asset::background(background_bytes);

    let engine = GameEngineBuilder::new()
        .window_title("Debug Physics Engine".to_string())
        .physics_engine(physics_engine)
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .sprite_sheet(sprite_sheet_asset)
        .background(background_asset)
        .build();

    engine.run();
}
