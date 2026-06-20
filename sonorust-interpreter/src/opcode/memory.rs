//! TODO: Reduce `int_from_f64_checked(..).unwrap_or_else(..)` call

use super::*;
use crate::{Value, util::int_from_f64_checked};

impl<E, M, S, T> Executable<E, M, S, T> for Get {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for GetShifted {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(self.x);
        let y = executor.execute(self.y);
        let s = executor.execute(self.s);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Set {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(self.value);

        executor
            .memory_access()
            .write(target_entity, block_id, index, value);

        value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetAdd {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(self.value);

        let current_value = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();

        let added_value = current_value + value;

        executor
            .memory_access()
            .write(target_entity, block_id, index, added_value);

        current_value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetMultiply {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(self.value);

        let current_value = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();

        let multiplied_value = current_value * value;

        executor
            .memory_access()
            .write(target_entity, block_id, index, multiplied_value);

        current_value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetShifted {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();

        let block_id = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(self.x);
        let y = executor.execute(self.y);
        let s = executor.execute(self.s);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(self.value);

        executor
            .memory_access()
            .write(target_entity, block_id, index, value);

        value
    }
}
