use sonorust_model::entity::shared_memory_array::{EntitySharedMemory, EntitySharedMemoryArray};

use crate::blocks::MemoryRegion;

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
