use bevy::ecs::resource::Resource;

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
