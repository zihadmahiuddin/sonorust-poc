pub mod archetype;
pub mod engine;
pub mod entity;
pub mod level;
pub mod runtime;
pub mod temporary;

pub trait MemoryRegion {
    fn size(&self) -> usize;
    fn read(&self, index: usize) -> Option<f64>;
    fn write(&mut self, index: usize, value: f64);
}
