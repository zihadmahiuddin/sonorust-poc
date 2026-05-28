use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct EntitySharedMemoryArray {
    pub items: Vec<EntitySharedMemory>,
}

impl EntitySharedMemoryArray {
    pub const ID: u16 = 4102;

    pub fn new(entity_count: usize) -> Self {
        Self {
            items: vec![EntitySharedMemory::default(); entity_count],
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntitySharedMemory(pub [f64; Self::SIZE]);

impl EntitySharedMemory {
    pub const ID: u16 = 4002;
    pub const SIZE: usize = 32;
}

impl Default for EntitySharedMemory {
    fn default() -> Self {
        Self([0.0; 32])
    }
}
