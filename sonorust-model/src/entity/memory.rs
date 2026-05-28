use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use super::EntityId;

#[derive(Debug, Clone)]
pub struct EntityMemory(pub [f64; Self::SIZE]);

impl EntityMemory {
    pub const SIZE: usize = 64;
}

impl Default for EntityMemory {
    fn default() -> Self {
        Self([0.0; 64])
    }
}

#[derive(Debug, Resource)]
pub struct EntityMemoryArray {
    pub items: BTreeMap<EntityId, EntityMemory>,
}

impl EntityMemoryArray {
    pub const ID: u16 = 4000;
}

impl EntityMemoryArray {
    pub fn new(entities: impl Iterator<Item = EntityId>) -> Self {
        Self {
            items: entities.map(|id| (id, EntityMemory::default())).collect(),
        }
    }
}
