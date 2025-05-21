use bevy::ecs::resource::Resource;

use crate::{Entity, EntityId, interpreter::memory::MemoryRegion};

#[derive(Debug, Resource)]
pub struct EntityInfoArray {
    items: Vec<EntityInfo>,
}

impl EntityInfoArray {
    pub const ID: u16 = 4103;

    pub fn new<'a>(entities: impl Iterator<Item = (&'a EntityId, &'a Entity)>) -> Self {
        Self {
            items: entities
                .map(|(entity_id, entity)| EntityInfo {
                    index: **entity_id,
                    archetype_index: entity.archetype_index,
                    state: EntityState::Waiting,
                })
                .collect(),
        }
    }

    pub fn entry(&self, entity_id: &EntityId) -> Option<&EntityInfo> {
        let item_index = **entity_id / EntityInfo::SIZE;
        self.items.get(item_index)
    }

    pub fn entry_mut(&mut self, entity_id: &EntityId) -> Option<&mut EntityInfo> {
        let item_index = **entity_id / EntityInfo::SIZE;
        self.items.get_mut(item_index)
    }
}

impl MemoryRegion for EntityInfoArray {
    fn size(&self) -> usize {
        self.items.iter().map(|item| item.size()).sum()
    }

    fn read(&self, index: usize) -> Option<f64> {
        let item_index = index / EntityInfo::SIZE;
        let index_in_item = index % EntityInfo::SIZE;
        self.entry(&EntityId(item_index))?.read(index_in_item)
    }

    fn write(&mut self, index: usize, value: f64) {
        let item_index = index / EntityInfo::SIZE;
        let index_in_item = index % EntityInfo::SIZE;
        if let Some(item) = self.entry_mut(&EntityId(item_index)) {
            item.write(index_in_item, value);
        }
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
    pub archetype_index: usize,
    pub state: EntityState,
}

impl EntityInfo {
    pub const ID: u16 = 4003;

    pub const SIZE: usize = 3;
}

impl MemoryRegion for EntityInfo {
    fn size(&self) -> usize {
        Self::SIZE
    }

    fn read(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.index as f64),
            1 => Some(self.archetype_index as f64),
            2 => Some(self.state.into()),
            _ => None,
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        match index {
            0 => self.index = value as usize,
            1 => self.archetype_index = value as usize,
            2 => self.state = value.try_into().unwrap(),
            _ => {}
        }
    }
}
