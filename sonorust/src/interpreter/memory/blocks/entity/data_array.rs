use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use crate::{Entity, EntityId, interpreter::memory::MemoryRegion};

#[derive(Debug, Resource)]
pub struct EntityDataArray {
    items: BTreeMap<EntityId, EntityData>,
}

impl EntityDataArray {
    pub const ID: u16 = 4101;

    pub fn new(entities: &BTreeMap<EntityId, Entity>) -> Self {
        Self {
            items: entities
                .iter()
                .map(|(id, e)| (*id, e.data.clone()))
                .collect(),
        }
    }
}

impl MemoryRegion for EntityDataArray {
    fn size(&self) -> usize {
        self.items.values().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityData::SIZE;
        let index_in_item = index % EntityData::SIZE;
        // dbg!(item_index, index_in_item);
        self.items.get(&EntityId(item_index))?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityData::SIZE;
        let index_in_item = index % EntityData::SIZE;
        if let Some(item) = self.items.get_mut(&EntityId(item_index)) {
            item.write(index_in_item, value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityData([f64; Self::SIZE]);

impl EntityData {
    pub const ID: u16 = 4001;
    pub const SIZE: usize = 32;

    pub fn new(data: [f64; Self::SIZE]) -> EntityData {
        Self(data)
    }
}

impl MemoryRegion for EntityData {
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

impl Default for EntityData {
    fn default() -> Self {
        Self([0.0; 32])
    }
}
