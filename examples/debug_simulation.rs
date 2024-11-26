extern crate game_engine;

use cgmath::Vector3;

use game_engine::engine::entity::{EntityBuilder, EntityComponentStorage, EntityHandle};
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::broadphase::BlockMap;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::collision::{RigidBody, RigidBodyBuilder, RigidBodyType};
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::constraint::resolver::elastic::ElasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::renderer_engine::asset::asset::Asset;
use game_engine::engine::renderer_engine::asset::font::{Font, Writer};
use game_engine::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use game_engine::engine::renderer_engine::post_process::PostProcessFilterId;
use game_engine::engine::renderer_engine::render_engine::RenderEngineControl;
use game_engine::engine::renderer_engine::{RenderBody, RenderBodyBuilder};
use game_engine::engine::util::color::{blue, green};
use game_engine::engine::util::zero;
use game_engine::engine::{PhysicsEngine, RenderEngine};

pub struct DebugPhysicsEngine<B>
where
    B: BroadPhase<Vec<CollisionCandidates>>,
{
    dt: f32,
    integrator: VerletIntegrator,
    constraint: Box<dyn Constraint>,
    broadphase: B,
    narrowphase: Box<dyn NarrowPhase>,
    ecs: EntityComponentStorage,
}

impl<B> DebugPhysicsEngine<B>
where
    B: BroadPhase<Vec<CollisionCandidates>>,
{
    pub fn new(window_size: &(u32, u32), broadphase: B) -> Self {
        let dt = 0.001;
        let mut ecs = EntityComponentStorage::new();
        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(0)
                        .velocity([2., 0., 0.])
                        .position([-400., 0., 0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .sprite_coord(SpriteCoordinate::new([2., 0.], [3., 1.]))
                        .build(),
                )
                .build(),
        );

        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(1)
                        .velocity([1., 2., 0.])
                        .position([400., 400., 0.])
                        .body_type(RigidBodyType::Circle { radius: 50. })
                        .build(),
                )
                .render_body(RenderBodyBuilder::new().color(blue()).build())
                .build(),
        );

        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(2)
                        .velocity([2., 1.5, 0.])
                        .position([350., 0., 0.])
                        .body_type(RigidBodyType::Circle { radius: 60. })
                        .build(),
                )
                .render_body(RenderBodyBuilder::new().color(green()).build())
                .build(),
        );

        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(3)
                        .velocity(zero())
                        .position(zero())
                        .body_type(RigidBodyType::Rectangle {
                            width: 100.,
                            height: 100.,
                        })
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .sprite_coord(SpriteCoordinate::new([1., 0.], [2., 1.]))
                        .build(),
                )
                .build(),
        );

        let integrator = VerletIntegrator::new(f32::MAX);

        let mut constraint = Box::new(BoxConstraint::new(ElasticConstraintResolver::new()));
        constraint.set_top_left(Vector3::new(
            -(window_size.0 as f32) / 2.0,
            window_size.1 as f32 / 2.0,
            0.0,
        ));
        constraint.set_bottom_right(Vector3::new(
            window_size.0 as f32 / 2.0,
            -(window_size.1 as f32) / 2.0,
            0.0,
        ));
        let narrowphase = Box::new(Naive::new(SimpleCollisionSolver::new()));

        Self {
            dt,
            integrator,
            constraint,
            broadphase,
            narrowphase,
            ecs,
        }
    }
}

impl<B> RenderEngine for DebugPhysicsEngine<B>
where
    B: BroadPhase<Vec<CollisionCandidates>>,
{
    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        let target_texture_handle = engine_ctl.request_texture_handle();

        let entities: Vec<EntityHandle> = self.ecs.entities_iter().collect();
        let rect_instances = game_engine::engine::util::get_rectangle_instances(&entities[..]);
        let circle_instances = game_engine::engine::util::get_circle_instances(&entities[..]);
        engine_ctl
            .render_background(&target_texture_handle)
            .unwrap();

        engine_ctl
            .render_rectangles(&target_texture_handle, &rect_instances, false)
            .unwrap();
        engine_ctl
            .render_circles(&target_texture_handle, &circle_instances, false)
            .unwrap();

        //let target_texture_handle = engine_ctl.run_post_process_filter(
        //    &PostProcessFilterId::Tint, &target_texture_handle).unwrap();

        let text_size = 110.;
        let text1 = Writer::write("HELLO WORLD", &[-400.0, -100.0, 0.0], text_size);
        let text2 = Writer::write("012 345 678 9", &[-700.0, -400.0, 0.0], text_size);
        engine_ctl
            .render_text(&target_texture_handle, text1, false)
            .unwrap();
        engine_ctl
            .render_text(&target_texture_handle, text2, false)
            .unwrap();

        //let target_texture_handle = engine_ctl
        //    .run_post_process_filter(&PostProcessFilterId::Gray, &target_texture_handle)
        //    .unwrap();
        engine_ctl
            .present(&target_texture_handle)
            .expect("Failed to present texture");
    }
}
impl<B> PhysicsEngine for DebugPhysicsEngine<B>
where
    B: BroadPhase<Vec<CollisionCandidates>>,
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

        let mut rigid_bodies: Vec<&mut RigidBody> = self.ecs.rigid_body_iter_mut().collect();
        let graphs: Vec<CollisionGraph> = candidates
            .iter()
            .filter_map(|c| self.narrowphase.collision_detection(&mut rigid_bodies, c))
            .collect();

        let rect_id = 3;
        let mut render_bodies: Vec<&mut RenderBody> = self.ecs.render_body_iter_mut().collect();
        render_bodies[rect_id].color = Vector3::new(0.0, 255.0, 255.0);
        for g in graphs {
            for node in g.collisions {
                if node.body_i_idx == rect_id || node.body_j_idx == rect_id {
                    render_bodies[rect_id].color = Vector3::new(255.0, 255.0, 0.0);
                }
            }
        }
    }

    fn get_bodies(&self) -> Vec<&RigidBody> {
        self.ecs.rigid_body_iter().collect()
    }
}

fn main() {
    let sprite_sheet_bytes = include_bytes!("../assets/sprite_sheet.png");
    let sprite_sheet_asset = Asset::sprite_sheet(sprite_sheet_bytes, 16, 16);

    let background_bytes = include_bytes!("../assets/background.png");
    let background_asset = Asset::background(background_bytes);

    let font = Font::new(
        include_bytes!("../src/engine/renderer_engine/asset/fonts/font.png"),
        11,
        11,
    );

    let window_size = (800, 800);
    let broadphase = BlockMap::new(window_size.0 as f32);
    let debug_engine = DebugPhysicsEngine::new(&window_size, broadphase);

    let engine = GameEngineBuilder::new()
        .window_title("Debug Physics Engine")
        .engine(debug_engine)
        .font(font)
        .add_post_process_filters(&mut vec![
            PostProcessFilterId::Gray,
            PostProcessFilterId::Tint,
        ])
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .sprite_sheet(sprite_sheet_asset)
        .background(background_asset)
        .build();

    engine.run();
}
