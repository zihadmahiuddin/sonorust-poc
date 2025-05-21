use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct EntitySharedMemoryArray {
    items: Vec<EntitySharedMemory>,
}

impl EntitySharedMemoryArray {
    pub const ID: u16 = 4102;

    pub fn new(entity_count: usize) -> Self {
        Self {
            items: vec![EntitySharedMemory::default(); entity_count],
        }
    }
}

impl MemoryRegion for EntitySharedMemoryArray {
    fn size(&self) -> usize {
        self.items.iter().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntitySharedMemory::SIZE;
        let index_in_item = index % EntitySharedMemory::SIZE;
        self.items.get(item_index)?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntitySharedMemory::SIZE;
        let index_in_item = index % EntitySharedMemory::SIZE;
        if let Some(item) = self.items.get_mut(item_index) {
            item.write(index_in_item, value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntitySharedMemory([f64; Self::SIZE]);

impl EntitySharedMemory {
    pub const ID: u16 = 4002;
    pub const SIZE: usize = 32;
}

impl MemoryRegion for EntitySharedMemory {
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

impl Default for EntitySharedMemory {
    fn default() -> Self {
        Self([0.0; 32])
    }
}
