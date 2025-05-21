use std::collections::BTreeSet;

use bevy::ecs::resource::Resource;

use crate::{EntityId, interpreter::memory::MemoryRegion};

#[derive(Debug, Default, Resource)]
pub struct EntityDespawn {
    items: BTreeSet<EntityId>,
}

impl EntityDespawn {
    pub const ID: u16 = 4004;
}

impl MemoryRegion for EntityDespawn {
    fn size(&self) -> usize {
        self.items.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        Some(self.items.get(&EntityId(index)).map(|_| 1.0).unwrap_or(0.0))
    }

    fn write(&mut self, index: usize, value: f64) {
        if value == 0.0 {
            self.remove(&EntityId(index));
        } else {
            self.add(EntityId(index));
        }
    }
}

impl EntityDespawn {
    pub fn iter(&self) -> impl Iterator<Item = &EntityId> {
        self.items.iter()
    }

    pub fn items_clone(&self) -> BTreeSet<EntityId> {
        self.items.clone()
    }

    pub fn add(&mut self, entity_id: EntityId) {
        self.items.insert(entity_id);
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.items.remove(entity_id);
    }
}
