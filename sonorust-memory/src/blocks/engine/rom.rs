use sonorust_model::engine::rom::EngineRom;

use crate::blocks::MemoryRegion;

impl MemoryRegion for EngineRom {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.0.get(index).copied()
    }

    fn write(&mut self, _index: usize, _value: f64) {}
}
