use super::*;
use crate::Value;

impl<E, M, S, T> Executable<E, M, S, T> for EaseInCubic {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);
        t * t * t
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for EaseOutCubic {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);
        1.0 - f64::powf(1.0 - t, 3.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for EaseInQuad {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);
        t * t
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for EaseOutQuad {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);
        1.0 - (1.0 - t) * (1.0 - t)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for EaseInOutQuad {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - f64::powf(-2.0 * t + 2.0, 2.0) / 2.0
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for EaseOutInQuad {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let t = executor.execute(self.value);

        if t < 0.5 {
            (1.0 - (1.0 - (t * 2.0)) * (1.0 - (t * 2.0))) / 2.0
        } else {
            let t_remapped = (t * 2.0) - 1.0;
            (t_remapped * t_remapped) / 2.0 + 0.5
        }
    }
}
