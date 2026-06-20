use super::*;
use tracing::warn;

impl<E, M, S, T> Executable<E, M, S, T> for HasSkinSprite {
    fn execute(&self, executor: &mut E) -> crate::Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO HasSkinSprite for {id}");
        1.0
    }
}
