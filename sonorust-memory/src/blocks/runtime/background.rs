use sonorust_model::runtime::background::RuntimeBackground;

use crate::blocks::MemoryRegion;

impl MemoryRegion for RuntimeBackground {
    fn size(&self) -> usize {
        8
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.x1),
            1 => Some(self.y1),
            2 => Some(self.x2),
            3 => Some(self.y2),
            4 => Some(self.x3),
            5 => Some(self.y3),
            6 => Some(self.x4),
            7 => Some(self.y4),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.x1 = value,
            1 => self.y1 = value,
            2 => self.x2 = value,
            3 => self.y2 = value,
            4 => self.x3 = value,
            5 => self.y3 = value,
            6 => self.x4 = value,
            7 => self.y4 = value,
            _ => {}
        }
    }
}
