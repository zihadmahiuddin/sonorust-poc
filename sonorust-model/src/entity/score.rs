use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use super::EntityId;

#[derive(Debug, Default, Resource)]
pub struct EntityScore {
    pub items: BTreeMap<EntityId, f64>,
}

impl EntityScore {
    pub const ID: u16 = 4006;
    pub const DEFAULT: f64 = 0.0;
}

impl EntityScore {
    pub fn new(entity_count: usize) -> Self {
        Self {
            items: (0..entity_count)
                .map(|i| (EntityId(i), EntityScore::DEFAULT))
                .collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&EntityId, &f64)> {
        self.items.iter()
    }

    pub fn insert(&mut self, entity_id: EntityId, value: f64) {
        self.items.insert(entity_id, value);
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.items.remove(entity_id);
    }
}
