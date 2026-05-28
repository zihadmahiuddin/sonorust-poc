use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for BeatToTime {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, beat) = executor.execute(self.beat);
        let time = executor.timing_access().beat_to_time(beat);
        (executor, time)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for BeatToBPM {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, beat) = executor.execute(self.beat);
        let bpm = executor.timing_access().beat_to_bpm(beat);
        (executor, bpm)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for TimeToScaledTime {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, time) = executor.execute(self.time);
        let scaled_time = executor.timing_access().time_to_scaled_time(time);
        (executor, scaled_time)
    }
}
