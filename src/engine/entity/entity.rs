use crate::engine::renderer_engine::RenderBody;
use crate::engine::RigidBody;

pub struct Entity {
    pub rigid_body: Option<RigidBody>,
    pub render_body: Option<RenderBody>,
}

pub struct EntityHandle<'a> {
    pub rigid_body: Option<&'a RigidBody>,
    pub render_body: Option<&'a RenderBody>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            rigid_body: None,
            render_body: None,
        }
    }
}

pub struct EntityBuilder {
    pub rigid_body: Option<RigidBody>,
    pub render_body: Option<RenderBody>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            rigid_body: None,
            render_body: None,
        }
    }

    pub fn rigid_body(mut self, rigid_body: RigidBody) -> Self {
        self.rigid_body = Some(rigid_body);
        self
    }

    pub fn render_body(mut self, render_body: RenderBody) -> Self {
        self.render_body = Some(render_body);
        self
    }

    pub fn build(self) -> Entity {
        Entity {
            rigid_body: self.rigid_body,
            render_body: self.render_body,
        }
    }
}
