use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct RuntimeTouchArray {
    pub touches: Vec<RuntimeTouch>,
}

impl RuntimeTouchArray {
    pub const ID: u16 = 1002;
}

#[derive(Debug)]
pub struct RuntimeTouch {
    pub id: f64,
    pub started: bool,
    pub ended: bool,
    pub time: f64,
    pub start_time: f64,
    pub x: f64,
    pub y: f64,
    pub start_x: f64,
    pub start_y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_r: f64,
    pub velocity_w: f64,
}

impl RuntimeTouch {
    pub const SIZE: usize = 15;
}
