use crate::util::int_from_f64_checked;

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Get {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (mut executor, index) = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let result = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for GetShifted {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (executor, x) = executor.execute(self.x);
        let (executor, y) = executor.execute(self.y);
        let (mut executor, s) = executor.execute(self.s);
        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let result = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Set {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (executor, index) = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let (mut executor, value) = executor.execute(self.value);
        executor
            .memory_access()
            .write(target_entity, block_id, index, value);
        (executor, value)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetAdd {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (executor, index) = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let (mut executor, value) = executor.execute(self.value);

        let current_value = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();
        let added_value = current_value + value;

        executor
            .memory_access()
            .write(target_entity, block_id, index, added_value);
        (executor, current_value)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetMultiply {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (executor, index) = executor.execute(self.index);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let (mut executor, value) = executor.execute(self.value);

        let current_value = executor
            .memory_access()
            .read(target_entity, block_id, index)
            .unwrap_or_default();
        let multiplied_value = current_value * value;

        executor
            .memory_access()
            .write(target_entity, block_id, index, multiplied_value);
        (executor, current_value)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SetShifted {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let target_entity = executor.target_entity();
        let (executor, block_id) = executor.execute(self.block_id);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));
        let (executor, x) = executor.execute(self.x);
        let (executor, y) = executor.execute(self.y);
        let (executor, s) = executor.execute(self.s);
        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));
        let (mut executor, value) = executor.execute(self.value);
        executor
            .memory_access()
            .write(target_entity, block_id, index, value);
        (executor, value)
    }
}
