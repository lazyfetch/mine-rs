use std::collections::HashMap;
use mc_protocol::entity::Entity;

#[derive(Default)]
pub struct EntityStorage {
    entities: HashMap<i32, Entity>
}

impl EntityStorage {
    pub fn get_entity_mut(&mut self, entity_id: i32) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_id)
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.insert(entity.id.0, entity);
    }

    pub fn remove(&mut self, key: &i32) {
        self.entities.remove(key);
    }
}