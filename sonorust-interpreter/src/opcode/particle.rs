use super::*;
use crate::Value;
use tracing::warn;

impl<E, M, S, T> Executable<E, M, S, T> for HasParticleEffect {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO HasParticleEffect for {id}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for DestroyParticleEffect {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        warn!("TODO DestroyParticleEffect for {id}");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for MoveParticleEffect {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let x1 = executor.execute(self.x1);
        let y1 = executor.execute(self.y1);
        let x2 = executor.execute(self.x2);
        let y2 = executor.execute(self.y2);
        let x3 = executor.execute(self.x3);
        let y3 = executor.execute(self.y3);
        let x4 = executor.execute(self.x4);
        let y4 = executor.execute(self.y4);
        warn!("TODO MoveParticleEffect for {id} ({x1},{y1}) ({x2},{y2}) ({x3},{y3}) ({x4},{y4})");
        0.0
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SpawnParticleEffect {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let id = executor.execute(self.id);
        let x1 = executor.execute(self.x1);
        let y1 = executor.execute(self.y1);
        let x2 = executor.execute(self.x2);
        let y2 = executor.execute(self.y2);
        let x3 = executor.execute(self.x3);
        let y3 = executor.execute(self.y3);
        let x4 = executor.execute(self.x4);
        let y4 = executor.execute(self.y4);
        let duration = executor.execute(self.duration);
        let is_looped = executor.execute(self.is_looped);

        warn!(
            "TODO MoveParticleEffect for {id} {duration} {is_looped} ({x1},{y1}) ({x2},{y2}) ({x3},{y3}) ({x4},{y4})"
        );

        0.0
    }
}
