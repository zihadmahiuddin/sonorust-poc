use sonorust_model::entity::{
    EntityId,
    input::{EntityInput, EntityInputArray},
};

use crate::blocks::MemoryRegion;

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
