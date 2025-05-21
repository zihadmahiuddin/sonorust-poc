use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct LevelLife {
    consecutive_perfect_increment: f64,
    consecutive_perfect_step: f64,
    consecutive_great_increment: f64,
    consecutive_great_step: f64,
    consecutive_good_increment: f64,
    consecutive_good_step: f64,
}

impl LevelLife {
    pub const ID: u16 = 2005;
}

impl MemoryRegion for LevelLife {
    fn size(&self) -> usize {
        6
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.consecutive_perfect_increment),
            1 => Some(self.consecutive_perfect_step),
            2 => Some(self.consecutive_great_increment),
            3 => Some(self.consecutive_great_step),
            4 => Some(self.consecutive_good_increment),
            5 => Some(self.consecutive_good_step),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.consecutive_perfect_increment = value,
            1 => self.consecutive_perfect_step = value,
            2 => self.consecutive_great_increment = value,
            3 => self.consecutive_great_step = value,
            4 => self.consecutive_good_increment = value,
            5 => self.consecutive_good_step = value,
            _ => {}
        }
    }
}
