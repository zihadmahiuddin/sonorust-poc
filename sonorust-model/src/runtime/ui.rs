use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct RuntimeUi {
    pub menu: RuntimeUiItem,
    pub judgment: RuntimeUiItem,
    pub combo_value: RuntimeUiItem,
    pub combo_text: RuntimeUiItem,
    pub primary_metric_bar: RuntimeUiItem,
    pub primary_metric_value: RuntimeUiItem,
    pub secondary_metric_bar: RuntimeUiItem,
    pub secondary_metric_value: RuntimeUiItem,
}

impl RuntimeUi {
    pub const ID: u16 = 1006;
}

#[derive(Debug, Default)]
pub struct RuntimeUiItem {
    pub anchor_x: f64,
    pub anchor_y: f64,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub alpha: f64,
    pub horizontal_alignment: HorizontalAlignment,
    pub background: f64,
}

impl RuntimeUiItem {
    pub const SIZE: usize = 10;
}

#[derive(Debug, Default, Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    #[default]
    Center,
    Right,
}

impl From<HorizontalAlignment> for f64 {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::Left => -1.0,
            HorizontalAlignment::Center => 0.0,
            HorizontalAlignment::Right => 1.0,
        }
    }
}

impl From<f64> for HorizontalAlignment {
    fn from(value: f64) -> Self {
        match value {
            -1.0 => HorizontalAlignment::Left,
            0.0 => HorizontalAlignment::Center,
            1.0 => HorizontalAlignment::Right,
            _ => unreachable!(),
        }
    }
}
