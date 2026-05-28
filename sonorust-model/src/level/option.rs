use bevy::ecs::resource::Resource;

use crate::engine::configuration::option::EngineOption;

#[derive(Debug, Resource)]
pub struct LevelOption {
    pub options: Vec<f64>,
}

impl LevelOption {
    pub const ID: u16 = 2002;

    pub fn new(options: &[EngineOption]) -> Self {
        Self {
            options: options
                .iter()
                .map(|option| match option {
                    EngineOption::Slider { def, name, .. } => {
                        if name == "#NOTE_SPEED" {
                            10.5
                        } else {
                            *def
                        }
                    }
                    EngineOption::Toggle { def, .. } => *def,
                    EngineOption::Select { def, .. } => *def,
                })
                .collect(),
        }
    }
}
