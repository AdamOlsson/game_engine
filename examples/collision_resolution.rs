use cgmath::Vector3;

use game_engine::engine::entity::{EntityBuilder, EntityComponentStorage, EntityHandle};
use game_engine::engine::event::mouse_input_event::{MouseButton, MouseInputEvent};
use game_engine::engine::event::user_event::UserEvent;
use game_engine::engine::event::ElementState;
use game_engine::engine::game_engine::GameEngineBuilder;
use game_engine::engine::physics_engine::broadphase::{BroadPhase, SpatialSubdivision};
use game_engine::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
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
use game_engine::engine::util::color::{blue, green, yellow};
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
        let mut ecs = EntityComponentStorage::new();

        ecs.add(
            EntityBuilder::new()
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(0)
                        .position([-200.0, 0.0, 0.0])
                        .body_type(RigidBodyType::Rectangle {
                            width: 100.0,
                            height: 200.0,
                        })
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .color(blue())
                        .shape(RenderBodyShape::Rectangle {
                            width: 100.0,
                            height: 200.0,
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
                        .position([200.0, 0.0, 0.0])
                        .body_type(RigidBodyType::Rectangle {
                            width: 100.0,
                            height: 200.0,
                        })
                        .build(),
                )
                .render_body(
                    RenderBodyBuilder::new()
                        .color(green())
                        .shape(RenderBodyShape::Rectangle {
                            width: 100.0,
                            height: 200.0,
                        })
                        .build(),
                )
                .build(),
        );

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
        if self.ecs.len() > 2 {
            let _ = self.ecs.remove_by_index(2);
        }

        self.integrator.update(
            self.ecs
                .rigid_body_iter_mut()
                .filter(|rb| rb.body_type != RigidBodyType::Unknown),
            self.dt,
        );

        self.ecs
            .rigid_body_iter_mut()
            .filter(|rb| rb.body_type != RigidBodyType::Unknown)
            .for_each(|b| self.constraint.apply_constraint(b));

        let candidates = self.broadphase.collision_detection(
            self.ecs
                .rigid_body_iter()
                .filter(|rb| rb.body_type != RigidBodyType::Unknown),
        );

        let pass1 = &candidates[0];
        let pass2 = &candidates[1];
        let pass3 = &candidates[2];
        let pass4 = &candidates[3];

        let collision_candidates = if pass1.len() > 0 {
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

        let candidates = match collision_candidates {
            Some(c) => c,
            None => return,
        };

        let mut bodies: Vec<&mut RigidBody> = self.ecs.rigid_body_iter_mut().collect();
        // TODO: Now the collision wont have any resolution as for rect rect only
        // detection is implemented
        // TODO: Make the object collide but they do not shoot away (crf = 0.0)
        let collision_graph = match self
            .narrowphase
            .collision_detection(&mut bodies, &candidates)
        {
            Some(graph) => graph,
            None => return,
        };

        let collision_info = &collision_graph.collisions[0].info;

        // TODO: wgpu instance buffers are only set to 2. How should the user specify the size of
        // the buffer?
        self.ecs.add(
            EntityBuilder::new()
                .render_body(
                    RenderBodyBuilder::new()
                        .shape(RenderBodyShape::Circle { radius: 10. })
                        .color(yellow())
                        .build(),
                )
                .rigid_body(
                    RigidBodyBuilder::default()
                        .id(2) // TODO: Remove
                        .body_type(RigidBodyType::Unknown)
                        .position(collision_info.collision_point)
                        .build(),
                )
                .build(),
        );
    }

    fn user_event(&mut self, event: UserEvent) {
        match event {
            UserEvent::Mouse(mouse_event) => match mouse_event {
                MouseInputEvent {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => {
                    self.cursor_state = ElementState::Pressed;
                    let bodies: Vec<&mut RigidBody> = self
                        .ecs
                        .rigid_body_iter_mut()
                        .filter(|rb| rb.body_type != RigidBodyType::Unknown)
                        .collect();
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
                        let mut bodies: Vec<&mut RigidBody> = self
                            .ecs
                            .rigid_body_iter_mut()
                            .filter(|rb| rb.body_type != RigidBodyType::Unknown)
                            .collect();
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
        let entities: Vec<EntityHandle> = self.ecs.entities_iter().collect();
        let rect_instances = game_engine::engine::util::get_rectangle_instances(&entities[..]);
        let circle_instances = game_engine::engine::util::get_circle_instances(&entities[..]);

        let texture_handle = engine_ctl.request_texture_handle();
        engine_ctl
            .render_rectangles(&texture_handle, &rect_instances, true)
            .expect("Failed to render circles");
        engine_ctl
            .render_circles(&texture_handle, &circle_instances, false)
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
        .max_num_rectangle_instances(5)
        .max_num_circle_instances(5)
        .build();

    engine.run();
}
