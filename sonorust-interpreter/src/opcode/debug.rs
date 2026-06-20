use super::*;
use crate::Value;

impl<E, M, S, T> Executable<E, M, S, T> for DebugLog {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        println!("Debug Log: {value}");
        value
    }
}
