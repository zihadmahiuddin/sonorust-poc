use sonorust_model::runtime::environment::RuntimeEnvironment;

use crate::blocks::MemoryRegion;

impl MemoryRegion for RuntimeEnvironment {
    fn size(&self) -> usize {
        5
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => {
                if self.debug_mode {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            1 => Some(self.screen_aspect_ratio),
            2 => Some(self.audio_offset),
            3 => Some(self.input_offset),
            4 => {
                if self.multiplayer {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => {
                if value == 1.0 {
                    self.debug_mode = true;
                } else if value == 0.0 {
                    self.debug_mode = false;
                }
            }
            1 => self.screen_aspect_ratio = value,
            2 => self.audio_offset = value,
            3 => self.input_offset = value,
            4 => {
                if value == 1.0 {
                    self.multiplayer = true;
                } else if value == 0.0 {
                    self.multiplayer = false;
                }
            }
            _ => {}
        }
    }
}
