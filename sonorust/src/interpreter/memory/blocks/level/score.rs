use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct LevelScore {
    perfect_multiplier: f64,
    great_multiplier: f64,
    good_multiplier: f64,
    consecutive_perfect_multiplier: f64,
    consecutive_perfect_step: f64,
    consecutive_perfect_cap: f64,
    consecutive_great_multiplier: f64,
    consecutive_great_step: f64,
    consecutive_great_cap: f64,
    consecutive_good_multiplier: f64,
    consecutive_good_step: f64,
    consecutive_good_cap: f64,
}

impl LevelScore {
    pub const ID: u16 = 2004;
}

impl MemoryRegion for LevelScore {
    fn size(&self) -> usize {
        12
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.perfect_multiplier),
            1 => Some(self.great_multiplier),
            2 => Some(self.good_multiplier),
            3 => Some(self.consecutive_perfect_multiplier),
            4 => Some(self.consecutive_perfect_step),
            5 => Some(self.consecutive_perfect_cap),
            6 => Some(self.consecutive_great_multiplier),
            7 => Some(self.consecutive_great_step),
            8 => Some(self.consecutive_great_cap),
            9 => Some(self.consecutive_good_multiplier),
            10 => Some(self.consecutive_good_step),
            11 => Some(self.consecutive_good_cap),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.perfect_multiplier = value,
            1 => self.great_multiplier = value,
            2 => self.good_multiplier = value,
            3 => self.consecutive_perfect_multiplier = value,
            4 => self.consecutive_perfect_step = value,
            5 => self.consecutive_perfect_cap = value,
            6 => self.consecutive_great_multiplier = value,
            7 => self.consecutive_great_step = value,
            8 => self.consecutive_great_cap = value,
            9 => self.consecutive_good_multiplier = value,
            10 => self.consecutive_good_step = value,
            11 => self.consecutive_good_cap = value,
            _ => {}
        }
    }
}
