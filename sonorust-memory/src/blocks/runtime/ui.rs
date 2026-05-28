use sonorust_model::runtime::ui::{RuntimeUi, RuntimeUiItem};

use crate::blocks::MemoryRegion;

impl MemoryRegion for RuntimeUi {
    fn size(&self) -> usize {
        self.menu.size()
            + self.judgment.size()
            + self.combo_value.size()
            + self.combo_text.size()
            + self.primary_metric_bar.size()
            + self.primary_metric_value.size()
            + self.secondary_metric_bar.size()
            + self.secondary_metric_value.size()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / RuntimeUiItem::SIZE;
        let index_in_item = index % RuntimeUiItem::SIZE;
        match item_index {
            0 => self.menu.read(index_in_item),
            1 => self.judgment.read(index_in_item),
            2 => self.combo_value.read(index_in_item),
            3 => self.combo_text.read(index_in_item),
            4 => self.primary_metric_bar.read(index_in_item),
            5 => self.primary_metric_value.read(index_in_item),
            6 => self.secondary_metric_bar.read(index_in_item),
            7 => self.secondary_metric_value.read(index_in_item),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / RuntimeUiItem::SIZE;
        let index_in_item = index % RuntimeUiItem::SIZE;
        match item_index {
            0 => self.menu.write(index_in_item, value),
            1 => self.judgment.write(index_in_item, value),
            2 => self.combo_value.write(index_in_item, value),
            3 => self.combo_text.write(index_in_item, value),
            4 => self.primary_metric_bar.write(index_in_item, value),
            5 => self.primary_metric_value.write(index_in_item, value),
            6 => self.secondary_metric_bar.write(index_in_item, value),
            7 => self.secondary_metric_value.write(index_in_item, value),
            _ => {}
        }
    }
}

impl MemoryRegion for RuntimeUiItem {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.anchor_x),
            1 => Some(self.anchor_y),
            2 => Some(self.pivot_x),
            3 => Some(self.pivot_y),
            4 => Some(self.width),
            5 => Some(self.height),
            6 => Some(self.rotation),
            7 => Some(self.alpha),
            8 => Some(self.horizontal_alignment.into()),
            9 => Some(self.background),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.anchor_x = value,
            1 => self.anchor_y = value,
            2 => self.pivot_x = value,
            3 => self.pivot_y = value,
            4 => self.width = value,
            5 => self.height = value,
            6 => self.rotation = value,
            7 => self.alpha = value,
            8 => self.horizontal_alignment = value.into(),
            9 => self.background = value,
            _ => {}
        }
    }
}
