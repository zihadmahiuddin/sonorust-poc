use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct RuntimeUiConfiguration {
    pub menu: RuntimeUiConfigurationItem,
    pub judgment: RuntimeUiConfigurationItem,
    pub combo: RuntimeUiConfigurationItem,
    pub primary_metric: RuntimeUiConfigurationItem,
    pub secondary_metric: RuntimeUiConfigurationItem,
}

impl RuntimeUiConfiguration {
    pub const ID: u16 = 1007;
}

#[derive(Debug)]
pub struct RuntimeUiConfigurationItem {
    pub scale: f64,
    pub alpha: f64,
}

impl RuntimeUiConfigurationItem {
    pub const SIZE: usize = 2;
}

impl Default for RuntimeUiConfigurationItem {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            scale: 1.0,
        }
    }
}
