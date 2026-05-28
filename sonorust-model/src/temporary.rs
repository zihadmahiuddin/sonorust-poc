use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct TemporaryMemory(pub [f64; 4096]);

impl Default for TemporaryMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}

impl TemporaryMemory {
    pub const ID: u16 = 10000;
}
