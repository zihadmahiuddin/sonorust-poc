use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Abs {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        (executor, value.abs())
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Negate {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        (executor, -value)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Add {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        self.inputs
            .iter()
            .fold((executor, 0.0), |(executor, last_result), idx| {
                let (executor, result) = executor.execute(*idx);
                (executor, result + last_result)
            })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Subtract {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return (executor, 0.0);
        };

        inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
            let (executor, value) = executor.execute(idx);
            (executor, acc - value)
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Multiply {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        self.inputs
            .iter()
            .fold((executor, 1.0), |(executor, last_result), idx| {
                let (executor, result) = executor.execute(*idx);
                (executor, result * last_result)
            })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Divide {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return (executor, 0.0);
        };

        inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
            let (executor, value) = executor.execute(idx);
            let result = if value != 0.0 {
                acc / value
            } else {
                // Decide what to do on division by zero
                todo!()
            };
            (executor, result)
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Mod {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return (executor, 0.0);
        };

        inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
            let (executor, value) = executor.execute(idx);
            let result = if value != 0.0 {
                acc % value
            } else {
                // Decide what to do on division by zero
                todo!()
            };
            (executor, result)
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Rem {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let (executor, first_value) = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return (executor, 0.0);
        };

        inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
            let (executor, value) = executor.execute(idx);
            let result = if value != 0.0 {
                acc % value
            } else {
                // Decide what to do on division by zero
                todo!()
            };
            (executor, result)
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Clamp {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, min) = executor.execute(self.min);
        let (executor, max) = executor.execute(self.max);
        let (executor, value) = executor.execute(self.value);
        let result = if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Lerp {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, min) = executor.execute(self.min);
        let (executor, max) = executor.execute(self.max);
        let (executor, value) = executor.execute(self.value);
        let result = min * (1.0 - value) + max * value;
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Unlerp {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, min) = executor.execute(self.min);
        let (executor, max) = executor.execute(self.max);
        let (executor, value) = executor.execute(self.value);
        let result = (value - min) / (max - min);
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for UnlerpClamped {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, min) = executor.execute(self.min);
        let (executor, max) = executor.execute(self.max);
        let (executor, value) = executor.execute(self.value);

        let t_factor = if min == max {
            0.0
        } else {
            (value - min) / (max - min)
        };

        let result = t_factor.clamp(0.0, 1.0);

        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Power {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        if self.inputs.is_empty() {
            return (executor, 0.0);
        }

        self.inputs
            .iter()
            .rfold((executor, 1.0), |(executor, last_result), idx| {
                let (executor, result) = executor.execute(*idx);
                (executor, result.powf(last_result))
            })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Min {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, x) = executor.execute(self.x);
        let (executor, y) = executor.execute(self.y);
        let result = x.min(y);
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Max {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, x) = executor.execute(self.x);
        let (executor, y) = executor.execute(self.y);
        let result = x.max(y);
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Remap {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, from_min) = executor.execute(self.from_min);
        let (executor, from_max) = executor.execute(self.from_max);
        let (executor, to_min) = executor.execute(self.to_min);
        let (executor, to_max) = executor.execute(self.to_max);
        let (executor, value) = executor.execute(self.value);
        let result = if from_max != from_min {
            (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
        } else {
            // TODO: decide what to do on division by zero
            to_min
        };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for RemapClamped {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, from_min) = executor.execute(self.from_min);
        let (executor, from_max) = executor.execute(self.from_max);
        let (executor, to_min) = executor.execute(self.to_min);
        let (executor, to_max) = executor.execute(self.to_max);
        let (executor, value) = executor.execute(self.value);
        let value = value.clamp(from_min.min(from_max), from_min.max(from_max));
        let result = if from_max != from_min {
            (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
        } else {
            // TODO: decide what to do on division by zero
            to_min
        };
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Round {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.round();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Floor {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.floor();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Ceil {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.ceil();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Trunc {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.trunc();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Sin {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.sin();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Cos {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.cos();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Arctan {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.atan();
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Arctan2 {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, x) = executor.execute(self.x);
        let (executor, y) = executor.execute(self.y);
        let result = x.atan2(y);
        (executor, result)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Log {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let (executor, value) = executor.execute(self.value);
        let result = value.ln();
        (executor, result)
    }
}
