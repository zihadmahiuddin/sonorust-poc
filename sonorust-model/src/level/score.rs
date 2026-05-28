use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct LevelScore {
    pub perfect_multiplier: f64,
    pub great_multiplier: f64,
    pub good_multiplier: f64,
    pub consecutive_perfect_multiplier: f64,
    pub consecutive_perfect_step: f64,
    pub consecutive_perfect_cap: f64,
    pub consecutive_great_multiplier: f64,
    pub consecutive_great_step: f64,
    pub consecutive_great_cap: f64,
    pub consecutive_good_multiplier: f64,
    pub consecutive_good_step: f64,
    pub consecutive_good_cap: f64,
}

impl LevelScore {
    pub const ID: u16 = 2004;
}
