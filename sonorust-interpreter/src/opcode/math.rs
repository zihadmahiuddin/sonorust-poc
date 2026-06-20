use bevy::math::FloatExt;
use std::ops::Neg;

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
        value.neg()
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
        self.inputs.iter().map(|idx| executor.execute(*idx)).sum()
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
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(*first);
        inputs.fold(init, |acc, &idx| acc - executor.execute(idx))
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
        self.inputs
            .iter()
            .map(|idx| executor.execute(*idx))
            .product()
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
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(*first);
        inputs.fold(init, |acc, &idx| acc / executor.execute(idx))
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
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(*first);
        inputs.fold(init, |acc, &idx| acc % executor.execute(idx))
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
        value.clamp(min, max)
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
        value.lerp(max, min)
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
        Value::inverse_lerp(min, max, value)
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

        if min == max {
            return 0.0;
        }

        Value::inverse_lerp(min, max, value).clamp(0.0, 1.0)
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

        self.inputs.iter().rfold(1.0, |acc, &idx| {
            let base = executor.execute(idx);
            base.powf(acc)
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

        if from_max == from_min {
            // TODO: decide what to do on division by zero
            return to_min;
        }

        executor
            .execute(self.value)
            .remap(from_min, from_max, to_min, to_max)
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

        if from_max == from_min {
            // TODO: decide what to do on division by zero
            return to_min;
        }

        executor
            .execute(self.value)
            .clamp(from_min.min(from_max), from_min.max(from_max))
            .remap(from_min, from_max, to_min, to_max)
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
