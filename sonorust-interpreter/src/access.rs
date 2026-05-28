pub use sonorust_memory::access::MemoryAccess;

use crate::side_effect::SideEffect;

pub trait SideEffectAccess {
    fn add(&mut self, side_effect: SideEffect);
}

pub trait TimingAccess {
    fn beat_to_time(&self, beat: f64) -> f64;
    fn beat_to_bpm(&self, beat: f64) -> f64;
    fn time_to_scaled_time(&self, time: f64) -> f64;
}
