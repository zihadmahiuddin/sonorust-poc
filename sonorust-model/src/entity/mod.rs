use bevy::prelude::*;

pub mod data_array;
pub mod despawn_array;
pub mod info;
pub mod input;
pub mod memory;
pub mod shared_memory_array;
pub mod life;
pub mod score;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct EntityId(pub usize);
