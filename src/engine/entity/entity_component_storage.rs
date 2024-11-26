use crate::engine::RigidBody;

pub struct Entity {
    pub rigid_body: Option<RigidBody>,
}

pub struct EntityComponentStorage {
    pub rigid_bodies: Vec<Option<RigidBody>>,
}

impl EntityComponentStorage {
    pub fn new() -> Self {
        Self {
            rigid_bodies: vec![],
        }
    }

    pub fn rigid_body_iter_mut(&mut self) -> impl Iterator<Item = &mut RigidBody> {
        self.rigid_bodies.iter_mut().filter_map(|rb| rb.as_mut())
    }

    pub fn rigid_body_iter(&self) -> impl Iterator<Item = &RigidBody> {
        self.rigid_bodies.iter().filter_map(|rb| rb.as_ref())
    }

    pub fn add(&mut self, entity: Entity) {
        // Note: Align all entities with None if the do not contain the component
        self.rigid_bodies.push(entity.rigid_body);
    }
}
