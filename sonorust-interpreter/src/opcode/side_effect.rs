use sonorust_model::archetype::ArchetypeId;

use crate::side_effect::{DrawSideEffect, SideEffect, SideEffectKind, SpawnSideEffect};
use crate::util::int_from_f64_checked;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Spawn {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (mut executor, archetype_id) = executor.execute(self.archetype_id);
        let archetype_id = ArchetypeId(int_from_f64_checked(archetype_id).unwrap_or_else(|| {
            panic!("Expected archetype ID to be valid usize, found {archetype_id}")
        }));

        let entity = executor.target_entity();
        executor.side_effect_access().add(SideEffect {
            entity,
            kind: SideEffectKind::Spawn(SpawnSideEffect { archetype_id }),
        });
        (executor, 0.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Draw {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, sprite_id) = executor.execute(self.sprite_id);
        let sprite_id = int_from_f64_checked(sprite_id)
            .unwrap_or_else(|| panic!("Expected sprite ID to be valid usize, found {sprite_id}"));

        let (executor, x1) = executor.execute(self.x1);
        let (executor, y1) = executor.execute(self.y1);
        let (executor, x2) = executor.execute(self.x2);
        let (executor, y2) = executor.execute(self.y2);
        let (executor, x3) = executor.execute(self.x3);
        let (executor, y3) = executor.execute(self.y3);
        let (executor, x4) = executor.execute(self.x4);
        let (executor, y4) = executor.execute(self.y4);
        let (executor, z) = executor.execute(self.z);
        let (mut executor, alpha) = executor.execute(self.alpha);

        let entity = executor.target_entity();
        executor.side_effect_access().add(SideEffect {
            entity,
            kind: SideEffectKind::Draw(DrawSideEffect {
                sprite_id,
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
                x4,
                y4,
                z,
                alpha,
            }),
        });
        (executor, 0.0)
    }
}
