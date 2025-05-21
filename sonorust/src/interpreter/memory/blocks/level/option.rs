use bevy::ecs::resource::Resource;

use crate::{engine_configuration::EngineOption, interpreter::memory::MemoryRegion};

#[derive(Debug, Resource)]
pub struct LevelOption {
    options: Vec<f64>,
}

impl LevelOption {
    pub const ID: u16 = 2002;

    pub fn new(options: &[EngineOption]) -> Self {
        Self {
            options: options
                .iter()
                .map(|option| match option {
                    EngineOption::Slider { def, .. } => *def,
                    EngineOption::Toggle { def, .. } => *def,
                    EngineOption::Select { def, .. } => *def,
                })
                .collect(),
        }
    }
}

impl MemoryRegion for LevelOption {
    fn size(&self) -> usize {
        self.options.len()
    }

    fn read(&self, index: usize) -> Option<f64> {
        self.options.get(index).copied()
    }

    fn write(&mut self, index: usize, value: f64) {
        if let Some(item) = self.options.get_mut(index) {
            *item = value;
        }
    }
}
