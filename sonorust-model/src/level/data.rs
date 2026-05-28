use serde::Deserialize;

use crate::archetype::data::{EngineArchetypeDataName, EngineArchetypeName};

use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct LevelDataMemory(pub [f64; 4096]);

impl LevelDataMemory {
    pub const ID: u16 = 2001;
}

impl Default for LevelDataMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelData {
    pub bgm_offset: f64,
    pub entities: Vec<LevelDataEntity>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelDataEntityData {
    pub name: EngineArchetypeDataName,
    #[serde(flatten)]
    pub payload: Option<LevelDataEntityDataPayload>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum LevelDataEntityDataPayload {
    Reference { reference: String },
    Value { value: f64 },
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelDataEntity {
    pub name: Option<String>,
    pub archetype: EngineArchetypeName,
    pub data: Vec<LevelDataEntityData>,
}
