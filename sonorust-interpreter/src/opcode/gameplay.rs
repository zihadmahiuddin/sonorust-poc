use tracing::warn;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Judge {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, source) = executor.execute(self.source);
        let (executor, target) = executor.execute(self.target);
        let (executor, min_perfect) = executor.execute(self.min_perfect);
        let (executor, max_perfect) = executor.execute(self.max_perfect);
        let (executor, min_great) = executor.execute(self.min_great);
        let (executor, max_great) = executor.execute(self.max_great);
        let (executor, min_good) = executor.execute(self.min_good);
        let (executor, max_good) = executor.execute(self.max_good);

        warn!(
            "TODO Judge for {source} {target} {min_perfect} {max_perfect} {min_great} {max_great} {min_good} {max_good}"
        );

        (executor, 1.0)
    }
}
