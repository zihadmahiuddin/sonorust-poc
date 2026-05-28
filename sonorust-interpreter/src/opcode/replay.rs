use tracing::warn;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for ExportValue {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, index) = executor.execute(self.index);
        let (executor, value) = executor.execute(self.value);
        warn!("TODO ExportValue for {index} {value}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StreamSet {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, key) = executor.execute(self.key);
        let (executor, value) = executor.execute(self.value);
        warn!("TODO ExportValue for {id} {key} {value}");
        (executor, 0.0)
    }
}
