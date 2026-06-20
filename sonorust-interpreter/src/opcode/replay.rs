use tracing::warn;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for ExportValue {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let index = executor.execute(self.index);
        let value = executor.execute(self.value);
        warn!("TODO ExportValue for {index} {value}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StreamSet {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let key = executor.execute(self.key);
        let value = executor.execute(self.value);
        warn!("TODO ExportValue for {id} {key} {value}");
        0.0
    }
}
