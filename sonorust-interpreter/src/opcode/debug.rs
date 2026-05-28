use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for DebugLog {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        println!("Debug Log: {value}");
        (executor, value)
    }
}
