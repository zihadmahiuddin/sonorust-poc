use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct RuntimeEnvironment {
    pub debug_mode: bool,
    pub screen_aspect_ratio: f64,
    pub audio_offset: f64,
    pub input_offset: f64,
    pub multiplayer: bool,
}

impl RuntimeEnvironment {
    pub const ID: u16 = 1000;
}
