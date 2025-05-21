// use super::MemoryRegion;
// use archetype::life::ArchetypeLife;
// use engine::rom::EngineRom;
// use entity::{
//     data_array::{EntityData, EntityDataArray}, despawn_array::EntityDespawn, info::{EntityInfo, EntityInfoArray}, memory::EntityMemoryArray, shared_memory_array::{EntitySharedMemory, EntitySharedMemoryArray}
// };
// use level::{
//     bucket::LevelBucket, data::LevelData, life::LevelLife, memory::LevelMemory,
//     option::LevelOption, score::LevelScore,
// };
// use runtime::{
//     background::RuntimeBackground, environment::RuntimeEnvironment,
//     particle_transform::RuntimeParticleTransform, skin_transform::RuntimeSkinTransform,
//     touch::RuntimeTouchArray, ui::RuntimeUi, ui_configuration::RuntimeUiConfiguration,
//     update::RuntimeUpdate,
// };
// use temporary::TemporaryMemory;

pub mod archetype;
pub mod engine;
pub mod entity;
pub mod level;
pub mod runtime;
pub mod temporary;

// macro_rules! memory_block {
//     ($($block_name:ident),+ $(,)?) => {
//         #[derive(Debug)]
//         pub enum MemoryBlock {
//             $($block_name($block_name)),+
//         }
//
//         impl MemoryBlock {
//             pub fn id(&self) -> u16 {
//                 match self {
//                     $(MemoryBlock::$block_name(_) => $block_name::ID),+,
//                     _ => unreachable!()
//                 }
//             }
//         }
//
//         impl MemoryRegion for MemoryBlock {
//             fn size(&self) -> usize {
//                 match self {
//                     $(MemoryBlock::$block_name(block) => block.size()),+,
//                     _ => unreachable!()
//                 }
//             }
//
//             fn read(&self, index: usize) -> Option<f64> {
//                 match self {
//                     $(MemoryBlock::$block_name(block) => block.read(index)),+,
//                     _ => unreachable!()
//                 }
//             }
//
//             fn write(&mut self, index: usize, value: f64) {
//                 match self {
//                     $(MemoryBlock::$block_name(block) => block.write(index, value)),+,
//                     _ => unreachable!()
//                 }
//             }
//         }
//     };
// }
//
// memory_block!(
//     RuntimeEnvironment,
//     RuntimeUpdate,
//     RuntimeTouchArray,
//     RuntimeSkinTransform,
//     RuntimeParticleTransform,
//     RuntimeBackground,
//     RuntimeUi,
//     RuntimeUiConfiguration,
//     LevelMemory,
//     LevelData,
//     LevelOption,
//     LevelBucket,
//     LevelScore,
//     LevelLife,
//     EngineRom,
//     EntityMemoryArray,
//     EntityData,
//     EntitySharedMemory,
//     EntityInfo,
//     EntityDespawn,
//     // EntityInput,
//     EntityDataArray,
//     EntitySharedMemoryArray,
//     EntityInfoArray,
//     ArchetypeLife,
//     TemporaryMemory,
// );
