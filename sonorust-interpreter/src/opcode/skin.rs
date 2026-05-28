use super::*;
use tracing::warn;

impl<E, M, S, T> Executable<E, M, S, T> for HasSkinSprite {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO HasSkinSprite for {id}");
        (executor, 1.0)
    }
}
