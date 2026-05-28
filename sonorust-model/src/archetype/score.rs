use std::collections::BTreeMap;

use bevy::ecs::resource::Resource;

use super::ArchetypeId;

#[derive(Debug, Default, Resource)]
pub struct ArchetypeScore {
    pub items: BTreeMap<ArchetypeId, f64>,
}

impl ArchetypeScore {
    pub const ID: u16 = 5001;
    pub const DEFAULT: f64 = 1.0;
}

impl ArchetypeScore {
    pub fn new(archetype_count: usize) -> Self {
        Self {
            items: (0..archetype_count)
                .map(|i| (ArchetypeId(i), ArchetypeScore::DEFAULT))
                .collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ArchetypeId, &f64)> {
        self.items.iter()
    }

    pub fn insert(&mut self, archetype_id: ArchetypeId, value: f64) {
        self.items.insert(archetype_id, value);
    }

    pub fn remove(&mut self, archetype_id: &ArchetypeId) {
        self.items.remove(archetype_id);
    }
}
