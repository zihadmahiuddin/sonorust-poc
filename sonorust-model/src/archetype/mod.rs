use bevy::prelude::*;

pub mod data;
pub mod life;
pub mod score;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct ArchetypeId(pub usize);
