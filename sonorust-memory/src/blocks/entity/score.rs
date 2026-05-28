use sonorust_model::entity::{EntityId, score::EntityScore};

use crate::blocks::MemoryRegion;

impl MemoryRegion for EntityScore {
    fn size(&self) -> usize {
        self.items.len() * std::mem::size_of::<f64>()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.items.get(&EntityId(index)).copied()
    }

    fn write(&mut self, index: usize, value: f64) {
        if let Some(item) = self.items.get_mut(&EntityId(index)) {
            *item = value;
        }
    }
}
