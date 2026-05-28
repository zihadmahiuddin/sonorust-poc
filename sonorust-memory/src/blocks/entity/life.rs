use sonorust_model::entity::life::{EntityLife, EntityLifeItem};

use crate::blocks::MemoryRegion;

impl MemoryRegion for EntityLife {
    fn size(&self) -> usize {
        self.items.iter().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityLifeItem::SIZE;
        let index_in_item = index % EntityLifeItem::SIZE;
        self.items.get(item_index)?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityLifeItem::SIZE;
        let index_in_item = index % EntityLifeItem::SIZE;
        if let Some(item) = self.items.get_mut(item_index) {
            item.write(index_in_item, value);
        }
    }
}

impl MemoryRegion for EntityLifeItem {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.perfect_life_increment),
            1 => Some(self.great_life_increment),
            2 => Some(self.good_life_increment),
            3 => Some(self.miss_life_increment),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.perfect_life_increment = value,
            1 => self.great_life_increment = value,
            2 => self.good_life_increment = value,
            3 => self.miss_life_increment = value,
            _ => {}
        }
    }
}
