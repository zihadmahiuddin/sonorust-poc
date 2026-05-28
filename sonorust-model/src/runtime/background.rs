use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct RuntimeBackground {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub x3: f64,
    pub y3: f64,
    pub x4: f64,
    pub y4: f64,
}

impl RuntimeBackground {
    pub const ID: u16 = 1005;
}
