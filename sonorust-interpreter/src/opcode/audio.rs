use tracing::warn;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Play {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, distance) = executor.execute(self.distance);
        warn!("TODO Play for {id} {distance}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayLooped {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO PlayLooped for {id}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayLoopedScheduled {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, time) = executor.execute(self.time);
        warn!("TODO PlayLoopedScheduled for {id}, {time}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayScheduled {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, time) = executor.execute(self.time);
        let (executor, distance) = executor.execute(self.distance);
        warn!("TODO PlayScheduled for {id}, {time}, {distance}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StopLoopedScheduled {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, time) = executor.execute(self.time);
        warn!("TODO StopLoopedScheduled for {id}, {time}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StopLooped {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO StopLooped for {id}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for HasEffectClip {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO HasEffectClip for {id}");
        (executor, 0.0)
    }
}
