use bevy::prelude::*;

pub mod data_array;
pub mod despawn_array;
pub mod info;
pub mod input;
pub mod life;
pub mod memory;
pub mod score;
pub mod shared_memory_array;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct EntityId(pub usize);
