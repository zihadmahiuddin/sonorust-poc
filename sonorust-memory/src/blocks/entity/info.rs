use sonorust_model::entity::{
    EntityId,
    info::{EntityInfo, EntityInfoArray},
};

use crate::blocks::MemoryRegion;

impl MemoryRegion for EntityInfoArray {
    fn size(&self) -> usize {
        self.items.values().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityInfo::SIZE;
        let index_in_item = index % EntityInfo::SIZE;
        self.entry(&EntityId(item_index))?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityInfo::SIZE;
        let index_in_item = index % EntityInfo::SIZE;
        if let Some(item) = self.entry_mut(&EntityId(item_index)) {
            item.write(index_in_item, value);
        }
    }
}

impl MemoryRegion for EntityInfo {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.index as f64),
            1 => Some(*self.archetype_id as f64),
            2 => Some(self.state.into()),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.index = value as usize,
            1 => *self.archetype_id = value as usize,
            2 => self.state = value.try_into().unwrap(),
            _ => {}
        }
    }
}
