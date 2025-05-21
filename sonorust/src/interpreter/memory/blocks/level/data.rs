use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct LevelData([f64; 4096]);

impl LevelData {
    pub const ID: u16 = 2001;
}

impl MemoryRegion for LevelData {
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

impl Default for LevelData {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}
