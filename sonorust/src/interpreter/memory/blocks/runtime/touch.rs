use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Default, Resource)]
pub struct RuntimeTouchArray {
    touches: Vec<RuntimeTouch>,
}

impl RuntimeTouchArray {
    pub const ID: u16 = 1002;
}

impl MemoryRegion for RuntimeTouchArray {
    fn size(&self) -> usize {
        self.touches.iter().map(|touch| touch.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let touch_index = index / RuntimeTouch::SIZE;
        let index_in_touch = index % RuntimeTouch::SIZE;
        self.touches.get(touch_index)?.read(index_in_touch)
    }

    fn write(&mut self, index: usize, value: f64) {
        let touch_index = index / RuntimeTouch::SIZE;
        let index_in_touch = index % RuntimeTouch::SIZE;
        if let Some(touch) = self.touches.get_mut(touch_index) {
            touch.write(index_in_touch, value);
        }
    }
}

#[derive(Debug)]
pub struct RuntimeTouch {
    id: f64,
    started: bool,
    ended: bool,
    time: f64,
    start_time: f64,
    x: f64,
    y: f64,
    start_x: f64,
    start_y: f64,
    delta_x: f64,
    delta_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    velocity_r: f64,
    velocity_w: f64,
}

impl RuntimeTouch {
    pub const SIZE: usize = 15;
}

impl MemoryRegion for RuntimeTouch {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.id),
            1 => {
                if self.started {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            2 => {
                if self.ended {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            3 => Some(self.time),
            4 => Some(self.start_time),
            5 => Some(self.x),
            6 => Some(self.y),
            7 => Some(self.start_x),
            8 => Some(self.start_y),
            9 => Some(self.delta_x),
            10 => Some(self.delta_y),
            11 => Some(self.velocity_x),
            12 => Some(self.velocity_y),
            13 => Some(self.velocity_r),
            14 => Some(self.velocity_w),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.id = value,
            1 => {
                if value == 1.0 {
                    self.started = true;
                } else if value == 0.0 {
                    self.started = false;
                }
            }
            2 => {
                if value == 1.0 {
                    self.ended = true;
                } else if value == 0.0 {
                    self.ended = false;
                }
            }
            3 => self.time = value,
            4 => self.start_time = value,
            5 => self.x = value,
            6 => self.y = value,
            7 => self.start_x = value,
            8 => self.start_y = value,
            9 => self.delta_x = value,
            10 => self.delta_y = value,
            11 => self.velocity_x = value,
            12 => self.velocity_y = value,
            13 => self.velocity_r = value,
            14 => self.velocity_w = value,
            _ => {}
        }
    }
}
