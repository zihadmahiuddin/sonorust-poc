use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct RuntimeUpdate {
    pub time: f64,
    pub delta_time: f64,
    pub scaled_time: f64,
    pub touch_count: f64,
}

impl RuntimeUpdate {
    pub const ID: u16 = 1001;
}

impl MemoryRegion for RuntimeUpdate {
    fn size(&self) -> usize {
        4
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.time),
            1 => Some(self.delta_time),
            2 => Some(self.scaled_time),
            3 => Some(self.touch_count),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.time = value,
            1 => self.delta_time = value,
            2 => self.scaled_time = value,
            3 => self.touch_count = value,
            _ => {}
        }
    }
}
