use super::*;
use crate::Value;

impl<E, M, S, T> Executable<E, M, S, T> for Equal {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if (lhs - rhs).abs() < f64::EPSILON {
            1.0
        } else {
            0.0
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for NotEqual {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if (lhs - rhs).abs() >= f64::EPSILON {
            1.0
        } else {
            0.0
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Greater {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if lhs > rhs { 1.0 } else { 0.0 }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for GreaterOr {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if lhs >= rhs { 1.0 } else { 0.0 }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Less {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if lhs < rhs { 1.0 } else { 0.0 }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for LessOr {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let lhs = executor.execute(self.lhs);
        let rhs = executor.execute(self.rhs);
        if lhs <= rhs { 1.0 } else { 0.0 }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for And {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut last_value = 0.0;

        for &input in &self.inputs {
            let value = executor.execute(input);

            if value == 0.0
            // && self.inputs[0] != 1171
            {
                return 0.0;
            }

            last_value = value;
        }

        last_value

        // let mut inputs_iter = self.inputs.iter();

        // let first_value = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let value = executor.execute(idx);
        //     (executor, (acc as i64 & value as i64) as f64)
        // })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Or {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        for &input in &self.inputs {
            let value = executor.execute(input);
            if value != 0.0 {
                return value;
            }
        }
        0.0

        // let mut inputs_iter = self.inputs.iter();

        // let first_value = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let value = executor.execute(idx);
        //     (executor, (acc as i64 | value as i64) as f64)
        // })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Not {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        if value == 0.0 { 1.0 } else { 0.0 }
    }
}
