use bevy::ecs::resource::Resource;

use crate::interpreter::memory::MemoryRegion;

#[derive(Debug, Resource)]
pub struct LevelBucket {
    pub buckets: Vec<LevelBucketItem>,
}

impl LevelBucket {
    pub const ID: u16 = 2003;

    pub(crate) fn new(bucket_count: usize) -> Self {
        Self {
            buckets: vec![LevelBucketItem::default(); bucket_count],
        }
    }
}

impl MemoryRegion for LevelBucket {
    fn size(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / LevelBucketItem::SIZE;
        let index_in_item = index % LevelBucketItem::SIZE;
        self.buckets.get(item_index)?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / LevelBucketItem::SIZE;
        let index_in_item = index % LevelBucketItem::SIZE;
        if let Some(bucket) = self.buckets.get_mut(item_index) {
            bucket.write(index_in_item, value);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LevelBucketItem {
    pub min_perfect_window: f64,
    pub max_perfect_window: f64,
    pub min_great_window: f64,
    pub max_great_window: f64,
    pub min_good_window: f64,
    pub max_good_window: f64,
}

impl LevelBucketItem {
    pub const SIZE: usize = 6;
}

impl MemoryRegion for LevelBucketItem {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.min_perfect_window),
            1 => Some(self.max_perfect_window),
            2 => Some(self.min_great_window),
            3 => Some(self.max_great_window),
            4 => Some(self.min_good_window),
            5 => Some(self.max_good_window),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.min_perfect_window = value,
            1 => self.max_perfect_window = value,
            2 => self.min_great_window = value,
            3 => self.max_great_window = value,
            4 => self.min_good_window = value,
            5 => self.max_good_window = value,
            _ => {}
        }
    }
}
