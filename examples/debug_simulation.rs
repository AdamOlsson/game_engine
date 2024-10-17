extern crate game_engine;

use cgmath::Vector3;
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use game_engine::engine::renderer_engine::asset::asset::Asset;
use game_engine::engine::renderer_engine::asset::font::{Font, Writer};
use game_engine::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use game_engine::engine::renderer_engine::post_process::PostProcessFilterId;
use game_engine::engine::renderer_engine::render_engine::RenderEngineControl;

use game_engine::engine::util::zero;
use game_engine::engine::util::color::{green, blue};
use game_engine::engine::{PhysicsEngine, RenderEngine};
use game_engine::engine::physics_engine::collision::rigid_body::{RigidBody, RigidBodyBuilder, RigidBodyType};
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::constraint::resolver::elastic::ElasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::broadphase::blockmap::BlockMap;

pub struct DebugPhysicsEngine {
    dt: f32,
    integrator: VerletIntegrator,
    constraint: Box<dyn Constraint>,
    broadphase: Box<dyn BroadPhase<Vec<CollisionCandidates>>>,
    narrowphase: Box<dyn NarrowPhase>,
}

impl DebugPhysicsEngine {
    pub fn new(window_size: &(u32,u32)) -> Self {
        let dt = 0.001;
        let bodies = vec![
            RigidBodyBuilder::default().id(0).velocity([2.,0.,0.]).position([-400.,0.,0.])
                .sprite_coord(SpriteCoordinate::new([2.,0.], [3.,1.]))
                .body_type(RigidBodyType::Circle { radius: 100.}).build(),
            
            RigidBodyBuilder::default().id(1).velocity([1.,2.,0.]).position([400.,400.,0.])
                .color(blue()).body_type(RigidBodyType::Circle { radius: 100.}).build(),

            RigidBodyBuilder::default().id(2).velocity([2.,1.5,0.]).position([350.,0.,0.])
                .color(green()).body_type(RigidBodyType::Circle { radius: 120.}).build(),

            RigidBodyBuilder::default().id(3).velocity(zero()).position(zero())
                .sprite_coord(SpriteCoordinate::new([1.,0.], [2.,1.]))
                .body_type(RigidBodyType::Rectangle { width: 200., height: 200. }).build(),
        ];

        let integrator = VerletIntegrator::new(f32::MAX, bodies);
        
        let mut constraint = Box::new(BoxConstraint::new(ElasticConstraintResolver::new()));
        constraint.set_top_left(Vector3::new(-(window_size.0 as f32), window_size.1 as f32, 0.0));
        constraint.set_bottom_right(Vector3::new(window_size.0 as f32, -(window_size.1 as f32), 0.0));
        let broadphase = Box::new(BlockMap::new(window_size.0 as f32));
        let narrowphase = Box::new(Naive::new(SimpleCollisionSolver::new()));

            
        Self { 
            dt, integrator, constraint, broadphase, narrowphase, }
    }
}

impl RenderEngine for DebugPhysicsEngine {
    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        let bodies = self.integrator.get_bodies();
        let target_texture_handle = engine_ctl.request_texture_handle();

        let rect_instances = game_engine::engine::util::get_rectangle_instances(bodies);
        let circle_instances = game_engine::engine::util::get_circle_instances(bodies);
        engine_ctl.render_background(&target_texture_handle).unwrap();

        engine_ctl.render_rectangles(&target_texture_handle, &rect_instances, false).unwrap();
        engine_ctl.render_circles(&target_texture_handle, &circle_instances, false).unwrap();
    
        //let target_texture_handle = engine_ctl.run_post_process_filter(
        //    &PostProcessFilterId::Tint, &target_texture_handle).unwrap();

        let text_size = 110.;
        let text1 = Writer::write("HELLO WORLD", &[-400.0, -100.0, 0.0], text_size);
        let text2 = Writer::write("012 345 678 9", &[-700.0, -400.0, 0.0], text_size);
        engine_ctl.render_text(&target_texture_handle, text1, false).unwrap();
        engine_ctl.render_text(&target_texture_handle, text2, false).unwrap();

        //let target_texture_handle= engine_ctl.run_post_process_filter(
        //    &PostProcessFilterId::Gray, &target_texture_handle).unwrap();
        engine_ctl.present(&target_texture_handle).expect("Failed to present texture");
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

    fn get_bodies(&self) -> &Vec<RigidBody> {
        &self.integrator.get_bodies()
    }
}

fn main() {
    let sprite_sheet_bytes = include_bytes!("../assets/sprite_sheet.png");
    let sprite_sheet_asset  = Asset::sprite_sheet(sprite_sheet_bytes, 16, 16);
    
    let background_bytes = include_bytes!("../assets/background.png"); // TODO
    let background_asset = Asset::background(background_bytes);

    let font = Font::new(include_bytes!("../src/engine/renderer_engine/asset/fonts/font.png"), 11, 11);

    let window_size = (800, 800);
    let debug_engine = DebugPhysicsEngine::new(&window_size);

    let engine = GameEngineBuilder::new()
        .window_title("Debug Physics Engine".to_string())
        .engine(debug_engine)
        .font(font)
        .add_post_process_filters(&mut vec![PostProcessFilterId::Gray, PostProcessFilterId::Tint])
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .sprite_sheet(sprite_sheet_asset)
        .background(background_asset)
        .build();

    engine.run();
}
