use super::*;
use tracing::warn;

impl<E, M, S, T> Executable<E, M, S, T> for HasParticleEffect {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO HasParticleEffect for {id}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for DestroyParticleEffect {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        warn!("TODO DestroyParticleEffect for {id}");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for MoveParticleEffect {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, x1) = executor.execute(self.x1);
        let (executor, y1) = executor.execute(self.y1);
        let (executor, x2) = executor.execute(self.x2);
        let (executor, y2) = executor.execute(self.y2);
        let (executor, x3) = executor.execute(self.x3);
        let (executor, y3) = executor.execute(self.y3);
        let (executor, x4) = executor.execute(self.x4);
        let (executor, y4) = executor.execute(self.y4);
        warn!("TODO MoveParticleEffect for {id} ({x1},{y1}) ({x2},{y2}) ({x3},{y3}) ({x4},{y4})");
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SpawnParticleEffect {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, id) = executor.execute(self.id);
        let (executor, x1) = executor.execute(self.x1);
        let (executor, y1) = executor.execute(self.y1);
        let (executor, x2) = executor.execute(self.x2);
        let (executor, y2) = executor.execute(self.y2);
        let (executor, x3) = executor.execute(self.x3);
        let (executor, y3) = executor.execute(self.y3);
        let (executor, x4) = executor.execute(self.x4);
        let (executor, y4) = executor.execute(self.y4);
        let (executor, duration) = executor.execute(self.duration);
        let (executor, is_looped) = executor.execute(self.is_looped);
        warn!(
            "TODO MoveParticleEffect for {id} {duration} {is_looped} ({x1},{y1}) ({x2},{y2}) ({x3},{y3}) ({x4},{y4})"
        );
        (executor, 0.0)
    }
}
