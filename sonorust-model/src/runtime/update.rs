use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct RuntimeUpdate {
    pub time: f64,
    pub delta_time: f64,
    pub scaled_time: f64,
    pub touch_count: f64,
}

impl RuntimeUpdate {
    pub const ID: u16 = 1001;
}
