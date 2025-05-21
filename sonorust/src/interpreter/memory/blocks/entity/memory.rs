use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use crate::{Entity, EntityId, interpreter::memory::MemoryRegion};

#[derive(Debug, Clone)]
pub struct EntityMemory([f64; Self::SIZE]);

impl EntityMemory {
    pub const SIZE: usize = 64;
}

impl MemoryRegion for EntityMemory {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.0.get(index).copied()
    }

    fn write(&mut self, index: usize, value: f64) {
        if let Some(item) = self.0.get_mut(index) {
            *item = value;
        }
    }
}

impl Default for EntityMemory {
    fn default() -> Self {
        Self([0.0; 64])
    }
}

#[derive(Debug, Resource)]
pub struct EntityMemoryArray {
    items: BTreeMap<EntityId, EntityMemory>,
}

impl EntityMemoryArray {
    pub const ID: u16 = 4000;
}

impl MemoryRegion for EntityMemoryArray {
    fn size(&self) -> usize {
        self.items.values().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityMemory::SIZE;
        let index_in_item = index % EntityMemory::SIZE;
        self.items.get(&EntityId(item_index))?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityMemory::SIZE;
        let index_in_item = index % EntityMemory::SIZE;
        if let Some(item) = self.items.get_mut(&EntityId(item_index)) {
            item.write(index_in_item, value);
        }
    }
}

impl EntityMemoryArray {
    pub fn new(entities: &BTreeMap<EntityId, Entity>) -> Self {
        Self {
            items: entities
                .keys()
                .map(|id| (*id, EntityMemory::default()))
                .collect(),
        }
    }
}
