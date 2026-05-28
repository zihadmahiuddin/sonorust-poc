use std::collections::BTreeMap;

use bevy::prelude::*;

use super::EntityId;

#[derive(Debug, Resource)]
pub struct EntityDataArray {
    pub items: BTreeMap<EntityId, EntityData>,
}

impl EntityDataArray {
    pub const ID: u16 = 4101;

    pub fn new(entities: impl Iterator<Item = (EntityId, EntityData)>) -> Self {
        Self {
            items: entities.collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityData(pub [f64; Self::SIZE]);

impl EntityData {
    pub const ID: u16 = 4001;
    pub const SIZE: usize = 32;

    pub fn new(data: [f64; Self::SIZE]) -> EntityData {
        Self(data)
    }
}
