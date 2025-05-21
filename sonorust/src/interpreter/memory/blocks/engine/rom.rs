use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct EngineRom(Vec<f64>);

impl EngineRom {
    pub const ID: u16 = 3000;
}

impl MemoryRegion for EngineRom {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.0.get(index).copied()
    }

    fn write(&mut self, _index: usize, _value: f64) {}
}
