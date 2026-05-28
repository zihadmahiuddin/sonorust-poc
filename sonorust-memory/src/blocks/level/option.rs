use sonorust_model::level::option::*;

use crate::blocks::MemoryRegion;

impl MemoryRegion for LevelOption {
    fn size(&self) -> usize {
        self.options.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.options.get(index).copied()
    }

    fn write(&mut self, index: usize, value: f64) {
        if let Some(item) = self.options.get_mut(index) {
            *item = value;
        }
    }
}
