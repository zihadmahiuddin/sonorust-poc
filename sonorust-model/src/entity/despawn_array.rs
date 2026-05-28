use std::collections::BTreeSet;

use bevy::ecs::resource::Resource;

use super::EntityId;

#[derive(Debug, Default, Resource)]
pub struct EntityDespawn {
    pub items: BTreeSet<EntityId>,
}

impl EntityDespawn {
    pub const ID: u16 = 4004;
}

impl EntityDespawn {
    pub fn iter(&self) -> impl Iterator<Item = &EntityId> {
        self.items.iter()
    }

    pub fn add(&mut self, entity_id: EntityId) {
        self.items.insert(entity_id);
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.items.remove(entity_id);
    }
}
