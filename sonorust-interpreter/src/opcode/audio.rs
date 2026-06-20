use crate::Value;
use tracing::warn;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Play {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let distance = executor.execute(self.distance);
        warn!("TODO Play for {id} {distance}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayLooped {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO PlayLooped for {id}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayLoopedScheduled {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let time = executor.execute(self.time);
        warn!("TODO PlayLoopedScheduled for {id}, {time}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for PlayScheduled {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let time = executor.execute(self.time);
        let distance = executor.execute(self.distance);
        warn!("TODO PlayScheduled for {id}, {time}, {distance}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StopLoopedScheduled {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let time = executor.execute(self.time);
        warn!("TODO StopLoopedScheduled for {id}, {time}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for StopLooped {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO StopLooped for {id}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for HasEffectClip {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO HasEffectClip for {id}");
        0.0
    }
}
