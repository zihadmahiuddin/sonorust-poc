use serde::Deserialize;

use crate::archetype::{EngineArchetypeDataName, EngineArchetypeName};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelDataJson {
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
