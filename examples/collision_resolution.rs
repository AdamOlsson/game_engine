use cgmath::Vector3;
use game_engine::engine::entity_component_storage::{Entity, EntityComponentStorage};
use game_engine::engine::event::mouse_input_event::{MouseButton, MouseInputEvent};
use game_engine::engine::event::user_event::UserEvent;
use game_engine::engine::event::ElementState;
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::broadphase::spatial_subdivision::spatial_subdivision::SpatialSubdivision;
use game_engine::engine::physics_engine::broadphase::BroadPhase;
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use game_engine::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBody;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBodyBuilder;
use game_engine::engine::physics_engine::collision::rigid_body::RigidBodyType;
use game_engine::engine::physics_engine::collision::CollisionGraph;
use game_engine::engine::physics_engine::constraint::box_constraint::BoxConstraint;
use game_engine::engine::physics_engine::constraint::resolver::inelastic::InelasticConstraintResolver;
use game_engine::engine::physics_engine::constraint::Constraint;
use game_engine::engine::physics_engine::integrator::verlet::VerletIntegrator;
use game_engine::engine::physics_engine::narrowphase::naive::Naive;
use game_engine::engine::physics_engine::narrowphase::NarrowPhase;
use game_engine::engine::renderer_engine::render_engine::RenderEngineControl;
use game_engine::engine::util;
use game_engine::engine::util::color::{blue, green};
use game_engine::engine::PhysicsEngine;
use game_engine::engine::RenderEngine;

struct CollisionResolution<C, B, N>
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
    cursor_state: ElementState,
    cursor_pos: (f32, f32),
    selected_body: usize,
    click_position_body_center_offset: (f32, f32),
}

impl<C, B, N> CollisionResolution<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    pub fn new(constraint: C, broadphase: B, narrowphase: N) -> Self {
        let dt = 0.001;
        let bodies = vec![
            RigidBodyBuilder::default()
                .id(0)
                .position([-200.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 100.0,
                    height: 200.0,
                })
                .color(blue())
                .build(),
            RigidBodyBuilder::default()
                .id(1)
                .position([200.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 100.0,
                    height: 200.0,
                })
                .color(green())
                .build(),
        ];

        let mut ecs = EntityComponentStorage::new();
        bodies.iter().for_each(|b| {
            ecs.add(Entity {
                rigid_body: Some(b.clone()),
            })
        });

        let integrator = VerletIntegrator::new(f32::MAX);
        let cursor_state = ElementState::Released;
        let cursor_pos = (0.0, 0.0);
        let click_position_body_center_offset = (0.0, 0.0);
        let selected_body = usize::MAX;
        return Self {
            dt,
            integrator,
            constraint,
            broadphase,
            narrowphase,
            ecs,
            cursor_state,
            cursor_pos,
            click_position_body_center_offset,
            selected_body,
        };
    }
}

impl<C, B, N> PhysicsEngine for CollisionResolution<C, B, N>
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

        // TODO: Display the collision information
        let candidates = self
            .broadphase
            .collision_detection(self.ecs.rigid_body_iter());

        let pass1 = &candidates[0];
        let pass2 = &candidates[1];
        let pass3 = &candidates[2];
        let pass4 = &candidates[3];

        let collision_info = if pass1.len() > 0 {
            Some(&pass1[0])
        } else if pass2.len() > 0 {
            Some(&pass2[0])
        } else if pass3.len() > 0 {
            Some(&pass3[0])
        } else if pass4.len() > 0 {
            Some(&pass4[0])
        } else {
            None
        };

        let mut _bodies: Vec<&mut RigidBody> = self.ecs.rigid_body_iter_mut().collect();
        if let Some(_info) = collision_info {
            // TODO: Display a renderbody circle
        }

        // TODO: Collide the objects but with 0.0 as crf
        //let _graphs_1: Vec<CollisionGraph> = pass1
        //    .iter()
        //    .filter_map(|c| self.narrowphase.collision_detection(bodies, c))
        //    .collect();
        //let _graphs_2: Vec<CollisionGraph> = pass2
        //    .iter()
        //    .filter_map(|c| self.narrowphase.collision_detection(bodies, c))
        //    .collect();
        //let _graphs_3: Vec<CollisionGraph> = pass3
        //    .iter()
        //    .filter_map(|c| self.narrowphase.collision_detection(bodies, c))
        //    .collect();
        //let _graphs_4: Vec<CollisionGraph> = pass4
        //    .iter()
        //    .filter_map(|c| self.narrowphase.collision_detection(bodies, c))
        //    .collect();
    }

    fn get_bodies(&self) -> Vec<&RigidBody> {
        self.ecs.rigid_body_iter().collect()
    }

    fn user_event(&mut self, event: UserEvent) {
        match event {
            UserEvent::Mouse(mouse_event) => match mouse_event {
                MouseInputEvent {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => {
                    self.cursor_state = ElementState::Pressed;
                    let bodies: Vec<&mut RigidBody> = self.ecs.rigid_body_iter_mut().collect();
                    self.selected_body = if bodies[0].click_inside(self.cursor_pos) {
                        0
                    } else if bodies[1].click_inside(self.cursor_pos) {
                        1
                    } else {
                        usize::MAX
                    };

                    if self.selected_body != usize::MAX {
                        self.click_position_body_center_offset = (
                            bodies[self.selected_body].position.x - self.cursor_pos.0,
                            bodies[self.selected_body].position.y - self.cursor_pos.1,
                        );
                    }
                }
                MouseInputEvent {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => {
                    self.cursor_state = ElementState::Released;
                    self.selected_body = usize::MAX;
                    self.click_position_body_center_offset = (0.0, 0.0);
                }

                _ => (),
            },
            UserEvent::CursorLeft => {
                self.cursor_state = ElementState::Released;
                self.selected_body = usize::MAX;
                self.click_position_body_center_offset = (0.0, 0.0);
            }
            UserEvent::CursorMoved(position) => {
                self.cursor_pos = (position.x as f32, position.y as f32);
                match self.cursor_state {
                    ElementState::Pressed => {
                        if self.selected_body == usize::MAX {
                            return;
                        }
                        let mut bodies: Vec<&mut RigidBody> =
                            self.ecs.rigid_body_iter_mut().collect();
                        let body = &mut bodies[self.selected_body];
                        let new_pos = Vector3::new(
                            self.cursor_pos.0 + self.click_position_body_center_offset.0,
                            self.cursor_pos.1 + self.click_position_body_center_offset.1,
                            0.0,
                        );
                        body.position = new_pos;
                        body.prev_position = new_pos;
                        body.velocity = Vector3::new(0.0, 0.0, 0.0);
                        self.constraint.apply_constraint(body);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl<C, B, N> RenderEngine for CollisionResolution<C, B, N>
where
    C: Constraint,
    B: BroadPhase<[Vec<CollisionCandidates>; 4]>,
    N: NarrowPhase + Sync,
{
    fn render(&mut self, engine_ctl: &mut RenderEngineControl) {
        let bodies = self.get_bodies();
        let circle_instances = util::get_circle_instances(&bodies[..]);
        let rect_instances = util::get_rectangle_instances(&bodies[..]);

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
        -(window_size.0 as f32) / 2.0,
        window_size.1 as f32 / 2.0,
        0.0,
    ));
    constraint.set_bottom_right(Vector3::new(
        window_size.0 as f32 / 2.0,
        -(window_size.1 as f32) / 2.0,
        0.0,
    ));
    let broadphase = SpatialSubdivision::new();
    let narrowphase = Naive::new(SimpleCollisionSolver::new());

    let collision_simulmation = CollisionResolution::new(constraint, broadphase, narrowphase);
    let engine = GameEngineBuilder::new()
        .window_title("Collision Simulation")
        .engine(collision_simulmation)
        .window_size(window_size)
        .target_frames_per_sec(60)
        .target_ticks_per_frame(1)
        .build();

    engine.run();
}
