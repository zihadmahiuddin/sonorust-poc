use sonorust_model::entity::{
    EntityId,
    data_array::{EntityData, EntityDataArray},
};

use crate::blocks::MemoryRegion;

impl MemoryRegion for EntityDataArray {
    fn size(&self) -> usize {
        self.items.values().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityData::SIZE;
        let index_in_item = index % EntityData::SIZE;
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
