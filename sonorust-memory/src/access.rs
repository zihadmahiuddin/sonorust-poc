use crate::blocks::MemoryRegion;
use bevy::prelude::*;
use sonorust_macros::generate_memory_access;
use sonorust_model::{
    archetype::{life::ArchetypeLife, score::ArchetypeScore},
    engine::rom::EngineRom,
    entity::{
        EntityId,
        data_array::{EntityData, EntityDataArray},
        despawn_array::EntityDespawn,
        info::{EntityInfo, EntityInfoArray},
        input::EntityInputArray,
        memory::{EntityMemory, EntityMemoryArray},
        shared_memory_array::{EntitySharedMemory, EntitySharedMemoryArray},
        life::EntityLife,
        score::EntityScore
    },
    level::{
        bucket::LevelBucket, data::LevelDataMemory, life::LevelLife, memory::LevelMemory,
        option::LevelOption, score::LevelScore,
    },
    runtime::{
        background::RuntimeBackground, environment::RuntimeEnvironment,
        particle_transform::RuntimeParticleTransform, skin_transform::RuntimeSkinTransform,
        touch::RuntimeTouchArray, ui::RuntimeUi, ui_configuration::RuntimeUiConfiguration,
        update::RuntimeUpdate,
    },
    temporary::TemporaryMemory,
};

pub trait MemoryAccess {
    fn read(&self, current_entity: EntityId, block_id: u16, index: usize) -> Option<f64>;
    fn write(&mut self, current_entity: EntityId, block_id: u16, index: usize, value: f64);
}

fn resolve_entity_index(block_id: u16, current_entity: EntityId, index: usize) -> (u16, usize) {
    match block_id {
        EntityData::ID => (
            EntityDataArray::ID,
            (*current_entity * EntityData::SIZE) + index,
        ),
        EntitySharedMemory::ID => (
            EntitySharedMemoryArray::ID,
            (*current_entity * EntitySharedMemory::SIZE) + index,
        ),
        EntityInfo::ID => (
            EntityInfoArray::ID,
            (*current_entity * EntityInfo::SIZE) + index,
        ),
        EntityMemoryArray::ID => (
            EntityMemoryArray::ID,
            (*current_entity * EntityMemory::SIZE) + index,
        ),
        EntityDespawn::ID => (EntityDespawn::ID, *current_entity + index),
        _ => (block_id, index),
    }
}

generate_memory_access!(
    [
        RuntimeEnvironment,
        RuntimeUpdate,
        RuntimeTouchArray,
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        RuntimeUi,
        RuntimeUiConfiguration,
        LevelMemory,
        LevelDataMemory,
        LevelOption,
        LevelBucket,
        LevelScore,
        LevelLife,
        EngineRom,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntityDataArray,
        EntitySharedMemoryArray,
        EntityInfoArray,
        EntityLife,
        EntityScore,
        ArchetypeLife,
        ArchetypeScore,
        TemporaryMemory,
    ]

    Preprocess: [
        RuntimeEnvironment,
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        RuntimeUi,
        RuntimeUiConfiguration,
        LevelMemory,
        LevelDataMemory,
        LevelBucket,
        LevelScore,
        LevelLife,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntityDataArray,
        EntitySharedMemoryArray,
        EntityLife,
        EntityScore,
        ArchetypeLife,
        ArchetypeScore,
        TemporaryMemory,
    ]
    SpawnOrder: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    ShouldSpawn: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    Initialize: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    UpdateSequential: [
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        LevelMemory,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntitySharedMemoryArray,
        TemporaryMemory,
    ]
    Touch: [
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        LevelMemory,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntitySharedMemoryArray,
        TemporaryMemory,
    ]
    UpdateParallel: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    Terminate: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
);
