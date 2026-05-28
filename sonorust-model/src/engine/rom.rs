use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct EngineRom(pub Vec<f64>);

impl EngineRom {
    pub const ID: u16 = 3000;
}
