use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::archetype::ArchetypeId;

use super::EntityId;

#[derive(Debug, Resource)]
pub struct EntityInfoArray {
    pub items: HashMap<EntityId, EntityInfo>,
}

impl EntityInfoArray {
    pub const ID: u16 = 4103;

    pub fn new<'a>(entities: impl Iterator<Item = (&'a EntityId, ArchetypeId)>) -> Self {
        Self {
            items: entities
                .map(|(entity_id, archetype_id)| {
                    (
                        *entity_id,
                        EntityInfo {
                            index: **entity_id,
                            archetype_id,
                            state: EntityState::Waiting,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn entry(&self, entity_id: &EntityId) -> Option<&EntityInfo> {
        self.items.get(entity_id)
    }

    pub fn entry_mut(&mut self, entity_id: &EntityId) -> Option<&mut EntityInfo> {
        self.items.get_mut(entity_id)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EntityState {
    #[default]
    Waiting,
    Active,
    Despawned,
}

impl TryFrom<f64> for EntityState {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(match value {
            0.0 => EntityState::Waiting,
            1.0 => EntityState::Active,
            2.0 => EntityState::Despawned,
            _ => return Err(()),
        })
    }
}

impl From<EntityState> for f64 {
    fn from(value: EntityState) -> Self {
        match value {
            EntityState::Waiting => 0.0,
            EntityState::Active => 1.0,
            EntityState::Despawned => 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub index: usize,
    pub archetype_id: ArchetypeId,
    pub state: EntityState,
}

impl EntityInfo {
    pub const ID: u16 = 4003;

    pub const SIZE: usize = 3;
}
