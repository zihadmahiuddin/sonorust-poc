use sonorust_model::archetype::{ArchetypeId, score::ArchetypeScore};

use crate::blocks::MemoryRegion;

impl MemoryRegion for ArchetypeScore {
    fn size(&self) -> usize {
        self.items.len() * std::mem::size_of::<f64>()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.items.get(&ArchetypeId(index)).copied()
    }

    fn write(&mut self, index: usize, value: f64) {
        if let Some(item) = self.items.get_mut(&ArchetypeId(index)) {
            *item = value;
        }
    }
}
