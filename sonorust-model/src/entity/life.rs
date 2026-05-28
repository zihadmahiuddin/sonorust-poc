use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct EntityLife {
    pub items: Vec<EntityLifeItem>,
}

impl EntityLife {
    pub const ID: u16 = 4007;

    pub fn new(entity_count: usize) -> Self {
        Self {
            items: vec![EntityLifeItem::default(); entity_count],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityLifeItem {
    pub perfect_life_increment: f64,
    pub great_life_increment: f64,
    pub good_life_increment: f64,
    pub miss_life_increment: f64,
}

impl EntityLifeItem {
    pub const SIZE: usize = 4;
}
