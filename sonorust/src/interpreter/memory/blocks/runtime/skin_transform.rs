use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct RuntimeSkinTransform([f64; 16]);

impl RuntimeSkinTransform {
    pub const ID: u16 = 1003;
}

impl MemoryRegion for RuntimeSkinTransform {
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
