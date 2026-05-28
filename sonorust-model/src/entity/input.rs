use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use super::EntityId;

#[derive(Debug, Clone)]
pub struct EntityInput {
    pub judgment: f64,
    pub accuracy: f64,
    pub bucket_index: f64,
    pub bucket_value: f64,
}

impl EntityInput {
    pub const SIZE: usize = 64;
}

impl Default for EntityInput {
    fn default() -> Self {
        Self {
            bucket_index: -1.0,
            accuracy: 0.0,
            bucket_value: 0.0,
            judgment: 0.0,
        }
    }
}

#[derive(Debug, Resource)]
pub struct EntityInputArray {
    pub items: BTreeMap<EntityId, EntityInput>,
}

impl EntityInputArray {
    pub const ID: u16 = 4005;
}

impl EntityInputArray {
    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities.map(|id| (*id, EntityInput::default())).collect(),
        }
    }
}
