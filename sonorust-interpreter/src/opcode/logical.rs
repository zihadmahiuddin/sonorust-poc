use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Equal {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if (lhs - rhs).abs() < f64::EPSILON {
            1.0
        } else {
            0.0
        };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for NotEqual {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if (lhs - rhs).abs() >= f64::EPSILON {
            1.0
        } else {
            0.0
        };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Greater {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if lhs > rhs { 1.0 } else { 0.0 };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for GreaterOr {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if lhs >= rhs { 1.0 } else { 0.0 };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Less {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if lhs < rhs { 1.0 } else { 0.0 };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for LessOr {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, lhs) = executor.execute(self.lhs);
        let (executor, rhs) = executor.execute(self.rhs);
        let result = if lhs <= rhs { 1.0 } else { 0.0 };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for And {
    fn execute(&self, mut executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut last_value = 0.0;

        for &input in &self.inputs {
            let (new_executor, value) = executor.execute(input);
            executor = new_executor;

            if value == 0.0
            // && self.inputs[0] != 1171
            {
                return (executor, 0.0);
            }

            last_value = value;
        }

        (executor, last_value)

        // let mut inputs_iter = self.inputs.iter();

        // let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let (executor, value) = executor.execute(idx);
        //     (executor, (acc as i64 & value as i64) as f64)
        // })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Or {
    fn execute(&self, mut executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        for &input in &self.inputs {
            let (new_executor, value) = executor.execute(input);
            executor = new_executor;

            if value != 0.0 {
                return (executor, value);
            }
        }

        (executor, 0.0)

        // let mut inputs_iter = self.inputs.iter();

        // let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let (executor, value) = executor.execute(idx);
        //     (executor, (acc as i64 | value as i64) as f64)
        // })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Not {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = if value == 0.0 { 1.0 } else { 0.0 };
        (executor, result)
    }
}
