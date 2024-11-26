use cgmath::Vector3;

use crate::engine::physics_engine::collision::rigid_body::{
    RigidBody, RigidBodyBuilder, RigidBodyType,
};
use crate::engine::renderer_engine::render_body::render_body::{RenderBody, RenderBodyBuilder};

pub struct Entity {
    pub body_type: RigidBodyType,
    pub rotation: f32,
    pub position: Vector3<f32>,

    pub rigid_body: Option<RigidBody>,
    pub render_body: Option<RenderBody>,
}

pub struct EntityBuilder {
    pub body_type: RigidBodyType,
    pub rotation: f32,
    pub position: Vector3<f32>,
    pub rigid_body_builder: Option<RigidBodyBuilder>,
    pub render_body_builder: Option<RenderBodyBuilder>,
}

impl std::default::Default for EntityBuilder {
    fn default() -> Self {
        let body_type = RigidBodyType::Unkown;
        let rotation = 0.0;
        let position = Vector3::new(0.0, 0.0, 0.0);
        let rigid_body_builder = None;
        let render_body_builder = None;
        Self {
            body_type,
            rotation,
            position,
            rigid_body_builder,
            render_body_builder,
        }
    }
}

impl EntityBuilder {
    pub fn body_type(mut self, body_type: RigidBodyType) -> Self {
        self.body_type = body_type;
        self
    }
    pub fn position(mut self, position: [f32; 3]) -> Self {
        self.position = position.into();
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn render_body_builder(mut self, render_body_builder: RenderBodyBuilder) -> Self {
        self.render_body_builder = Some(render_body_builder);
        self
    }

    pub fn rigid_body_builder(mut self, rigid_body_builder: RigidBodyBuilder) -> Self {
        self.rigid_body_builder = Some(rigid_body_builder);
        self
    }

    pub fn build(self) -> Entity {
        let rigid_body = if let Some(rbb) = self.rigid_body_builder {
            Some(rbb.build())
        } else {
            None
        };
        let render_body = if let Some(rbb) = self.render_body_builder {
            Some(rbb.build())
        } else {
            None
        };
        Entity {
            rigid_body,
            render_body,
            body_type: self.body_type,
            rotation: self.rotation,
            position: self.position,
        }
    }
}
