use sonorust_model::entity::{EntityId, despawn_array::EntityDespawn};

use crate::blocks::MemoryRegion;

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
