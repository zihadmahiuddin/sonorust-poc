use tracing::warn;

use super::*;
use crate::Value;

impl<E, M, S, T> Executable<E, M, S, T> for Judge {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let source = executor.execute(self.source);
        let target = executor.execute(self.target);
        let min_perfect = executor.execute(self.min_perfect);
        let max_perfect = executor.execute(self.max_perfect);
        let min_great = executor.execute(self.min_great);
        let max_great = executor.execute(self.max_great);
        let min_good = executor.execute(self.min_good);
        let max_good = executor.execute(self.max_good);

        warn!(
            "TODO Judge for {source} {target} {min_perfect} {max_perfect} {min_great} {max_great} {min_good} {max_good}"
        );

        1.0
    }
}
