use bevy::ecs::{component::Component, event::Event};
use sonorust_model::{archetype::ArchetypeId, entity::EntityId};

#[derive(Event, Debug, Clone)]
pub struct SideEffect {
    pub entity: EntityId,
    pub kind: SideEffectKind,
}

#[derive(Debug, Clone)]
pub struct SpawnSideEffect {
    pub archetype_id: ArchetypeId,
}

#[derive(Debug, Component, Clone)]
pub struct DrawSideEffect {
    pub sprite_id: usize,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub x3: f64,
    pub y3: f64,
    pub x4: f64,
    pub y4: f64,
    pub z: f64,
    pub alpha: f64,
}

#[derive(Debug, Clone)]
pub enum SideEffectKind {
    Spawn(SpawnSideEffect),
    Draw(DrawSideEffect),
}
