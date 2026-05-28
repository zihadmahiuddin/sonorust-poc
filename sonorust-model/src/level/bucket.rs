use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct LevelBucket {
    pub buckets: Vec<LevelBucketItem>,
}

impl LevelBucket {
    pub const ID: u16 = 2003;

    pub fn new(bucket_count: usize) -> Self {
        Self {
            buckets: vec![LevelBucketItem::default(); bucket_count],
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
