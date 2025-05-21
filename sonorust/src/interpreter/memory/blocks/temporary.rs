use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct TemporaryMemory([f64; 4096]);

impl Default for TemporaryMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}

impl TemporaryMemory {
    pub const ID: u16 = 10000;
}

impl MemoryRegion for TemporaryMemory {
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
