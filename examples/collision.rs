
use core::f32;

use cgmath::Vector3;
use game_engine::engine::physics_engine::broadphase::spatial_subdivision::spatial_subdivision::SpatialSubdivision;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBodyBuilder;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBodyType;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::constraint::resolver::inelastic::InelasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::util;
use game_engine::engine::util::color::blue;
use game_engine::engine::util::color::red;
use game_engine::engine::util::zero;
use game_engine::engine::PhysicsEngine;
use game_engine::engine::RenderEngine;
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBody;
use game_engine::engine::renderer_engine::render_engine::RenderEngineControl;
use rand::Rng;

const NUM_ROWS: usize = 1;
const NUM_COLS: usize = 1;
const RADIUS: f32 = 80.0;

//fn spawn_bodies(
//    radius: f32,
//    acceleration: Vector3<f32>,
//    columns: usize,
//    rows: usize,
//) -> Vec<RigidBody> {
//    let spacing_dist = radius / 2.0;
//    let color = Vector3::new(255.0, 0.0, 0.0);
//    let velocity = Vector3::zero();
//
//    let mut bodies = Vec::new();
//    let mut rng = rand::thread_rng();
//    let spacing = (radius * 2.0) + spacing_dist;
//
//    for row in 0..rows {
//        for col in 0..columns {
//            let base_x = (col as f32) * spacing - (columns as f32 * spacing) / 2.0;
//            let base_y = (row as f32) * spacing - (rows as f32 * spacing) / 2.0;
//            
//            let variance_x: f32 = rng.gen_range(-1.0..1.0);
//            let variance_y: f32 = rng.gen_range(-1.0..1.0);
//            
//            let position = Vector3::new(base_x + variance_x, base_y + variance_y, 0.0);
//            
//            let body = RigidBody::circle(
//                row * columns + col, 
//                velocity,
//                acceleration,
//                position.clone(),
//                position,
//                radius,
//                color,
//            );
//            
//            bodies.push(body);
//        }
//    }
//    
//    bodies
//}

struct Collision <C, B, N>
    where 
        C: Constraint,
        B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
        N: NarrowPhase + Sync
{
    dt: f32,
    integrator: VerletIntegrator,
    constraint: C,
    broadphase: B,
    narrowphase: N,
}

impl <C, B, N> Collision<C, B, N>
    where 
        C: Constraint,
        B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
        N: NarrowPhase + Sync
{
    pub fn new(constraint: C, broadphase: B, narrowphase: N) -> Self {
        let dt = 0.001;
        //let acceleration = Vector3::new(0., (-9.82 / dt)*60., 0.);
        //let bodies = spawn_bodies(RADIUS, acceleration, NUM_COLS, NUM_ROWS);
        // TODO:
        // - Refactor CircleRect collision to handle rotation
        // - RectRect collision
        // - Refactor CircleCircle collision using techniques in RectCircle and RectRect
        // - For RectCircle collision (and probably RectRect and CircleCircle) we perform the collision 
        //      detection twice. Refactor this
        // - Add rotation to CircleCircle, CircleRect and RectRect collisions
        let bodies = vec![
            RigidBodyBuilder::default().id(0).velocity([10.,0.,0.]).position([-400.0,0.,0.])
                .color(red()).body_type(RigidBodyType::Circle { radius: 50.0 }).build(),
            RigidBodyBuilder::default().id(1).velocity(zero()).position(zero())
                .color(blue()).body_type(RigidBodyType::Rectangle { width: 100., height: 100.0 })
                .build(),
        ];
        
        let integrator = VerletIntegrator::new(f32::MAX, bodies);
            
        return Self { dt, integrator, constraint, broadphase,narrowphase}
    }
}

impl <C, B, N> PhysicsEngine for Collision<C, B, N> 
where 
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync
{
    fn update(&mut self) {
        self.integrator.update(self.dt);
        let bodies: &mut Vec<RigidBody> = self.integrator.get_bodies_mut();
        bodies.iter_mut().for_each(|b| self.constraint.apply_constraint(b));

        let candidates = self.broadphase.collision_detection(&bodies);

        let pass1 = &candidates[0];
        let pass2 = &candidates[1];
        let pass3 = &candidates[2];
        let pass4 = &candidates[3];

        let _graphs_1: Vec<CollisionGraph> = pass1.iter()
            .map(|c| self.narrowphase.collision_detection(bodies, c))
            .collect();
        let _graphs_2: Vec<CollisionGraph> = pass2.iter()
            .map(|c| self.narrowphase.collision_detection(bodies, c))
            .collect();
        let _graphs_3: Vec<CollisionGraph> = pass3.iter()
            .map(|c| self.narrowphase.collision_detection(bodies, c))
            .collect();
        let _graphs_4: Vec<CollisionGraph> = pass4.iter()
            .map(|c| self.narrowphase.collision_detection(bodies, c))
            .collect();

    }

    fn get_bodies(&self) -> &Vec<RigidBody> {
        self.integrator.get_bodies()
    }
}

impl <C, B, N> RenderEngine for Collision<C, B, N> 
where 
        C: Constraint,
        B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
        N: NarrowPhase + Sync
{    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        let bodies = self.get_bodies();
        let circle_instances = util::get_circle_instances(bodies);
        let rect_instances = util::get_rectangle_instances(bodies);

        let texture_handle = engine_ctl.request_texture_handle();
        engine_ctl.render_circles(&texture_handle, &circle_instances, true)
            .expect("Failed to render circles");
        engine_ctl.render_rectangles(&texture_handle, &rect_instances, false)
            .expect("Failed to render circles");
        engine_ctl.present(&texture_handle)
            .expect("Failed to present texture");
    }
}

fn main() {
    let window_size = (800,800);
    //let mut constraint = BoxConstraint::new(ElasticConstraintResolver::new());
    let mut constraint = BoxConstraint::new(InelasticConstraintResolver::new());
    constraint.set_top_left(Vector3::new(-(window_size.0 as f32), window_size.1 as f32, 0.0));
    constraint.set_bottom_right(Vector3::new(window_size.0 as f32, -(window_size.1 as f32), 0.0));
    let broadphase = SpatialSubdivision::new();
    let narrowphase = Naive::new(SimpleCollisionSolver::new());

    let collision_simulmation = Collision::new(constraint, broadphase, narrowphase);
    let engine = GameEngineBuilder::new()
        .window_title("Collision Simulation".to_string())
        .engine(collision_simulmation)
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .build();

    engine.run();
}
