use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

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

impl MemoryRegion for RuntimeUiConfiguration {
    fn size(&self) -> usize {
        self.menu.size()
            + self.judgment.size()
            + self.combo.size()
            + self.primary_metric.size()
            + self.secondary_metric.size()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / RuntimeUiConfigurationItem::SIZE;
        let index_in_item = index % RuntimeUiConfigurationItem::SIZE;
        match item_index {
            0 => self.menu.read(index_in_item),
            1 => self.judgment.read(index_in_item),
            2 => self.combo.read(index_in_item),
            3 => self.primary_metric.read(index_in_item),
            4 => self.secondary_metric.read(index_in_item),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / RuntimeUiConfigurationItem::SIZE;
        let index_in_item = index % RuntimeUiConfigurationItem::SIZE;
        match item_index {
            0 => self.menu.write(index_in_item, value),
            1 => self.judgment.write(index_in_item, value),
            2 => self.combo.write(index_in_item, value),
            3 => self.primary_metric.write(index_in_item, value),
            4 => self.secondary_metric.write(index_in_item, value),
            _ => {}
        }
    }
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

impl MemoryRegion for RuntimeUiConfigurationItem {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.scale),
            1 => Some(self.alpha),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.scale = value,
            1 => self.alpha = value,
            _ => {}
        }
    }
}
