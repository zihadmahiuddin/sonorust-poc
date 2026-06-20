use super::*;
use crate::Value;

impl<E, M, S, T> Executable<E, M, S, T> for Abs {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.abs()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Negate {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        -value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Add {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        self.inputs.iter().fold(0.0, |acc, idx| {
            let result = executor.execute(*idx);
            result + acc
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Subtract {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let first_value = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return 0.0;
        };

        inputs_iter.fold(first_value, |acc, &idx| {
            let value = executor.execute(idx);
            acc - value
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Multiply {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        self.inputs.iter().fold(1.0, |acc, idx| {
            let result = executor.execute(*idx);
            result * acc
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Divide {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let first_value = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return 0.0;
        };

        inputs_iter.fold(first_value, |acc, &idx| {
            let value = executor.execute(idx);
            if value != 0.0 {
                acc / value
            } else {
                // Decide what to do on division by zero
                todo!()
            }
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Mod {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let first_value = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return 0.0;
        };

        inputs_iter.fold(first_value, |acc, &idx| {
            let value = executor.execute(idx);
            if value != 0.0 {
                acc % value
            } else {
                // Decide what to do on division by zero
                todo!()
            }
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Rem {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut inputs_iter = self.inputs.iter();

        let first_value = if let Some(first_node) = inputs_iter.next() {
            executor.execute(*first_node)
        } else {
            return 0.0;
        };

        inputs_iter.fold(first_value, |acc, &idx| {
            let value = executor.execute(idx);
            if value != 0.0 {
                acc % value
            } else {
                // Decide what to do on division by zero
                todo!()
            }
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Clamp {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let min = executor.execute(self.min);
        let max = executor.execute(self.max);
        let value = executor.execute(self.value);
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Lerp {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let min = executor.execute(self.min);
        let max = executor.execute(self.max);
        let value = executor.execute(self.value);
        min * (1.0 - value) + max * value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Unlerp {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let min = executor.execute(self.min);
        let max = executor.execute(self.max);
        let value = executor.execute(self.value);
        (value - min) / (max - min)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for UnlerpClamped {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let min = executor.execute(self.min);
        let max = executor.execute(self.max);
        let value = executor.execute(self.value);

        let t_factor = if min == max {
            0.0
        } else {
            (value - min) / (max - min)
        };

        t_factor.clamp(0.0, 1.0)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Power {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        if self.inputs.is_empty() {
            return 0.0;
        }

        self.inputs.iter().rfold(1.0, |acc, idx| {
            let result = executor.execute(*idx);
            result.powf(acc)
        })
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Min {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let x = executor.execute(self.x);
        let y = executor.execute(self.y);
        x.min(y)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Max {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let x = executor.execute(self.x);
        let y = executor.execute(self.y);
        x.max(y)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Remap {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let from_min = executor.execute(self.from_min);
        let from_max = executor.execute(self.from_max);
        let to_min = executor.execute(self.to_min);
        let to_max = executor.execute(self.to_max);
        let value = executor.execute(self.value);

        if from_max != from_min {
            (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
        } else {
            // TODO: decide what to do on division by zero
            to_min
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for RemapClamped {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let from_min = executor.execute(self.from_min);
        let from_max = executor.execute(self.from_max);
        let to_min = executor.execute(self.to_min);
        let to_max = executor.execute(self.to_max);
        let value = executor.execute(self.value);

        let value = value.clamp(from_min.min(from_max), from_min.max(from_max));
        if from_max != from_min {
            (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
        } else {
            // TODO: decide what to do on division by zero
            to_min
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Round {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.round()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Floor {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.floor()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Ceil {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.ceil()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Trunc {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.trunc()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Sin {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.sin()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Cos {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.cos()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Arctan {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.atan()
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Arctan2 {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let x = executor.execute(self.x);
        let y = executor.execute(self.y);
        x.atan2(y)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Log {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let value = executor.execute(self.value);
        value.ln()
    }
}
