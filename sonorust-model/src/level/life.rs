use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct LevelLife {
    pub consecutive_perfect_increment: f64,
    pub consecutive_perfect_step: f64,
    pub consecutive_great_increment: f64,
    pub consecutive_great_step: f64,
    pub consecutive_good_increment: f64,
    pub consecutive_good_step: f64,
}

impl LevelLife {
    pub const ID: u16 = 2005;
}
