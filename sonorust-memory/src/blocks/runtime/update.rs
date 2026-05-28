use sonorust_model::runtime::update::RuntimeUpdate;

use crate::blocks::MemoryRegion;

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
