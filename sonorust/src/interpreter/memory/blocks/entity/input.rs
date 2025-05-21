use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use crate::{EntityId, interpreter::memory::MemoryRegion};

#[derive(Debug, Clone)]
pub struct EntityInput {
    judgment: f64,
    accuracy: f64,
    bucket_index: f64,
    bucket_value: f64,
}

impl EntityInput {
    pub const SIZE: usize = 64;
}

impl Default for EntityInput {
    fn default() -> Self {
        Self {
            bucket_index: -1.0,
            accuracy: 0.0,
            bucket_value: 0.0,
            judgment: 0.0,
        }
    }
}

impl MemoryRegion for EntityInput {
    fn size(&self) -> usize {
        4
    }

    fn read(&self, index: usize) -> Option<f64> {
        Some(match index {
            0 => self.judgment,
            1 => self.accuracy,
            2 => self.bucket_index,
            3 => self.bucket_value,
            _ => return None,
        })
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.judgment = value,
            1 => self.accuracy = value,
            2 => self.bucket_index = value,
            3 => self.bucket_value = value,
            _ => {}
        }
    }
}

#[derive(Debug, Resource)]
pub struct EntityInputArray {
    items: BTreeMap<EntityId, EntityInput>,
}

impl EntityInputArray {
    pub const ID: u16 = 4005;
}

impl MemoryRegion for EntityInputArray {
    fn size(&self) -> usize {
        self.items.values().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityInput::SIZE;
        let index_in_item = index % EntityInput::SIZE;
        self.items.get(&EntityId(item_index))?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityInput::SIZE;
        let index_in_item = index % EntityInput::SIZE;
        if let Some(item) = self.items.get_mut(&EntityId(item_index)) {
            item.write(index_in_item, value);
        }
    }
}

impl EntityInputArray {
    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities.map(|id| (*id, EntityInput::default())).collect(),
        }
    }
}
