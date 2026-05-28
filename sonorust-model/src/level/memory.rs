use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct LevelMemory(pub [f64; 4096]);

impl LevelMemory {
    pub const ID: u16 = 2000;
}

impl Default for LevelMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}
