use super::{Entity, EntityHandle};

use crate::engine::renderer_engine::RenderBody;
use crate::engine::RigidBody;

pub struct EntityComponentStorage {
    pub rigid_bodies: Vec<Option<RigidBody>>,
    pub render_bodies: Vec<Option<RenderBody>>,
}

impl EntityComponentStorage {
    pub fn new() -> Self {
        Self {
            rigid_bodies: vec![],
            render_bodies: vec![],
        }
    }

    pub fn rigid_body_iter_mut(&mut self) -> impl Iterator<Item = &mut RigidBody> {
        self.rigid_bodies.iter_mut().filter_map(|rb| rb.as_mut())
    }

    pub fn rigid_body_iter(&self) -> impl Iterator<Item = &RigidBody> {
        self.rigid_bodies.iter().filter_map(|rb| rb.as_ref())
    }

    pub fn render_body_iter_mut(&mut self) -> impl Iterator<Item = &mut RenderBody> {
        self.render_bodies.iter_mut().filter_map(|rb| rb.as_mut())
    }

    pub fn render_body_iter(&self) -> impl Iterator<Item = &RenderBody> {
        self.render_bodies.iter().filter_map(|rb| rb.as_ref())
    }

    pub fn entities_iter(&self) -> impl Iterator<Item = EntityHandle> {
        std::iter::zip(self.rigid_bodies.iter(), self.render_bodies.iter()).map(
            |(rigid, render)| EntityHandle {
                rigid_body: rigid.as_ref(),
                render_body: render.as_ref(),
            },
        )
    }

    pub fn add(&mut self, entity: Entity) {
        // Note: Align all entities with None if the do not contain the component
        self.rigid_bodies.push(entity.rigid_body);
        self.render_bodies.push(entity.render_body);
    }
}
