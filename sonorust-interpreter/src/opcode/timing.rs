use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for BeatToTime {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let beat = executor.execute(self.beat);
        executor.timing_access().beat_to_time(beat)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for BeatToBPM {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let beat = executor.execute(self.beat);
        executor.timing_access().beat_to_bpm(beat)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for TimeToScaledTime {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let time = executor.execute(self.time);
        executor.timing_access().time_to_scaled_time(time)
    }
}
