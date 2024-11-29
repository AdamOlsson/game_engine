use core::f32;

use cgmath::Vector3;
use game_engine::engine::entity::{EntityBuilder, EntityComponentStorage, EntityHandle};
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::broadphase::SpatialSubdivision;
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::collision::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::{RigidBody, RigidBodyBuilder, RigidBodyType};
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::constraint::resolver::inelastic::InelasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::renderer_engine::{
    RenderBodyBuilder, RenderBodyShape, RenderEngineControl,
};
use game_engine::engine::util::color::blue;
use game_engine::engine::util::color::red;
use game_engine::engine::PhysicsEngine;
use game_engine::engine::RenderEngine;

struct Collision<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    dt: f32,
    integrator: VerletIntegrator,
    constraint: C,
    broadphase: B,
    narrowphase: N,
    ecs: EntityComponentStorage,
}

impl<C, B, N> Collision<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    pub fn new(constraint: C, broadphase: B, narrowphase: N) -> Self {
        let dt = 0.001;
        //let acceleration = Vector3::new(0., (-9.82 / dt)*60., 0.);
        //let bodies = spawn_bodies(RADIUS, acceleration, NUM_COLS, NUM_ROWS);
        // TODO:
        // - RectRect collision
        // - Refactor CircleCircle collision using techniques in RectCircle and RectRect
        // - Box constraint should handle rotation as well
        // - Move restitution to the rigid body and determine effective restitution
        //      using weighted average during collision
        let mut ecs = EntityComponentStorage::new();
        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(0)
                        .velocity([0., 0., 0.])
                        .position([0., 5., 0.])
                        .body_type(RigidBodyType::Rectangle {
                            width: 500.,
                            height: 100.0,
                        })
                        .rotational_velocity(std::f32::consts::PI / 120.0)
                        //.rotation(std::f32::consts::PI/2.0)
                        .mass(1.)
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .color(blue())
                        .shape(RenderBodyShape::Rectangle {
                            width: 500.,
                            height: 100.,
                        })
                        .build(),
                )
                .build(),
        );

        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(1)
                        .velocity([0., 3., 0.])
                        .position([-200., -200., 0.])
                        .mass(1.)
                        .body_type(RigidBodyType::Circle { radius: 50.0 })
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .color(red())
                        .shape(RenderBodyShape::Circle { radius: 50.0 })
                        .build(),
                )
                .build(),
        );

        let integrator = VerletIntegrator::new(f32::MAX);

        return Self {
            dt,
            integrator,
            constraint,
            broadphase,
            narrowphase,
            ecs,
        };
    }
}

impl<C, B, N> PhysicsEngine for Collision<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    fn update(&mut self) {
        self.integrator
            .update(self.ecs.rigid_body_iter_mut(), self.dt);

        self.ecs
            .rigid_body_iter_mut()
            .for_each(|b| self.constraint.apply_constraint(b));

        let candidates = self
            .broadphase
            .collision_detection(self.ecs.rigid_body_iter());

        let pass1 = &candidates[0];
        let pass2 = &candidates[1];
        let pass3 = &candidates[2];
        let pass4 = &candidates[3];

        let mut bodies: Vec<&mut RigidBody> = self.ecs.rigid_body_iter_mut().collect();
        let _graphs_1: Vec<CollisionGraph> = pass1
            .iter()
            .filter_map(|c| self.narrowphase.collision_detection(&mut bodies, c))
            .collect();
        let _graphs_2: Vec<CollisionGraph> = pass2
            .iter()
            .filter_map(|c| self.narrowphase.collision_detection(&mut bodies, c))
            .collect();
        let _graphs_3: Vec<CollisionGraph> = pass3
            .iter()
            .filter_map(|c| self.narrowphase.collision_detection(&mut bodies, c))
            .collect();
        let _graphs_4: Vec<CollisionGraph> = pass4
            .iter()
            .filter_map(|c| self.narrowphase.collision_detection(&mut bodies, c))
            .collect();

        //panic!();
        //if _graphs_1.len() != 0 || _graphs_2.len() != 0 || _graphs_3.len() != 0 || _graphs_3.len() != 0 {
        //panic!();
        //}
    }
}

impl<C, B, N> RenderEngine for Collision<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        let entities: Vec<EntityHandle> = self.ecs.entities_iter().collect();
        let rect_instances = game_engine::engine::util::get_rectangle_instances(&entities[..]);
        let circle_instances = game_engine::engine::util::get_circle_instances(&entities[..]);

        let texture_handle = engine_ctl.request_texture_handle();
        engine_ctl
            .render_circles(&texture_handle, &circle_instances, true)
            .expect("Failed to render circles");
        engine_ctl
            .render_rectangles(&texture_handle, &rect_instances, false)
            .expect("Failed to render circles");
        engine_ctl
            .present(&texture_handle)
            .expect("Failed to present texture");
    }
}

fn main() {
    let window_size = (800, 800);

    let mut constraint = BoxConstraint::new(InelasticConstraintResolver::new());
    constraint.set_top_left(Vector3::new(
        -(window_size.0 as f32 / 2.0),
        window_size.1 as f32 / 2.0,
        0.0,
    ));
    constraint.set_bottom_right(Vector3::new(
        window_size.0 as f32 / 2.0,
        -(window_size.1 as f32 / 2.0),
        0.0,
    ));
    let broadphase = SpatialSubdivision::new();
    let narrowphase = Naive::new(SimpleCollisionSolver::new());

    let collision_simulmation = Collision::new(constraint, broadphase, narrowphase);
    let engine = GameEngineBuilder::new()
        .window_title("Collision Simulation")
        .engine(collision_simulmation)
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .max_num_circle_instances(10)
        .max_num_rectangle_instances(10)
        .build();

    engine.run();
}
