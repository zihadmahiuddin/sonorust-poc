use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct ArchetypeLife {
    pub items: Vec<ArchetypeLifeItem>,
}

impl ArchetypeLife {
    pub const ID: u16 = 5000;

    pub fn new(archetype_count: usize) -> Self {
        Self {
            items: vec![ArchetypeLifeItem::default(); archetype_count],
        }
    }
}

impl MemoryRegion for ArchetypeLife {
    fn size(&self) -> usize {
        self.items.iter().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / ArchetypeLifeItem::SIZE;
        let index_in_item = index % ArchetypeLifeItem::SIZE;
        self.items.get(item_index)?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / ArchetypeLifeItem::SIZE;
        let index_in_item = index % ArchetypeLifeItem::SIZE;
        if let Some(item) = self.items.get_mut(item_index) {
            item.write(index_in_item, value);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ArchetypeLifeItem {
    pub perfect_life_increment: f64,
    pub great_life_increment: f64,
    pub good_life_increment: f64,
    pub miss_life_increment: f64,
}

impl ArchetypeLifeItem {
    pub const SIZE: usize = 4;
}

impl MemoryRegion for ArchetypeLifeItem {
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
