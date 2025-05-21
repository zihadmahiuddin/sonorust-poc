use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct LevelMemory([f64; 4096]);

impl LevelMemory {
    pub const ID: u16 = 2000;
}

impl MemoryRegion for LevelMemory {
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

impl Default for LevelMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}
