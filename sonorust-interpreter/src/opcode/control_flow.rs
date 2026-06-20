use std::ops::ControlFlow;

use crate::{Value, util::int_from_f64_checked};

use super::*;

impl<E, M, S, T> Executable<E, M, S, T> for Execute {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let mut last_result: f64 = 0.0;

        for &node_index in &self.nodes {
            // Check the program flow before executing the next operation.
            match executor.control() {
                ControlFlow::Break(_) => {
                    // If a break is active, stop executing further opcodes in this sequence.
                    // The Block containing this sequence will handle the break state.
                    return last_result; // Or the break value if accessible
                }
                ControlFlow::Continue(()) => {
                    // Proceed to execute the current operation.
                }
            }
            last_result = executor.execute(node_index);
        }
        last_result
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for If {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let test = executor.execute(self.test);
        let target = if test != 0.0 {
            self.consequent
        } else {
            self.alternate
        };
        executor.execute(target)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Block {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let result = match executor.control_mut() {
            ControlFlow::Break(stack) if stack.len() == 1 => Some((0.0, ControlFlow::Continue(()))),
            ControlFlow::Break(stack) => {
                let last = stack.pop().unwrap_or_default();
                Some((last, ControlFlow::Break(stack.clone())))
            }
            _ => None,
        };

        if let Some((value, control)) = result {
            executor.with_control(control);
            return value;
        }

        executor.execute(self.body)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for Break {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let count = executor.execute(self.count);
        let count = int_from_f64_checked(count)
            .unwrap_or_else(|| panic!("Expected break count to be valid usize, found {count}"));

        let value = executor.execute(self.value);
        let mut stack = match executor.control() {
            ControlFlow::Continue(()) => vec![],
            ControlFlow::Break(stack) => stack.clone(),
        };

        for _ in 0usize..count {
            stack.push(value);
        }
        executor.with_control(ControlFlow::Break(stack));
        value
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for While {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        loop {
            let test = executor.execute(self.test);

            if test == 0.0 {
                return 0.0;
            }

            let _ = executor.execute(self.body);

            match executor.control_mut() {
                ControlFlow::Break(stack) => {
                    return stack.pop().unwrap_or_default();
                }
                ControlFlow::Continue(()) => {}
            }
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SwitchInteger {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        // dbg!(&self);
        // dbg!(archetype);
        let discriminant = executor.execute(self.discriminant);
        // let discriminant = discriminant.round();
        let discriminant: usize = int_from_f64_checked(discriminant).unwrap_or_else(|| {
            panic!(
                "Expected SwitchIntegerWith discriminant to be valid usize, found {discriminant}"
            )
        });

        if let Some(consequent_index) = self.consequents.get(discriminant) {
            executor.execute(*consequent_index)
        } else {
            0.0
        }
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SwitchIntegerWithDefault {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let discriminant = executor.execute(self.discriminant);
        let discriminant = discriminant.round(); // TODO: investigate why discriminant is 0.5 on pjsk engine...
        let Some(discriminant) = int_from_f64_checked::<usize>(discriminant) else {
            return executor.execute(self.default_consequent);
        };

        let consequent_index = *self
            .consequents
            .get(discriminant)
            .unwrap_or(&self.default_consequent);

        executor.execute(consequent_index)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for SwitchWithDefault {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let discriminant = executor.execute(self.discriminant);
        let discriminant = discriminant.round();
        let discriminant: usize = int_from_f64_checked(discriminant).unwrap_or_else(|| {
            panic!(
                "Expected SwitchWithDefault discriminant to be valid usize, found {discriminant}"
            )
        });

        let mut result_f64: f64;

        for chunk in self.tests_and_consequents.chunks_exact(2) {
            let test = chunk[0];
            let consequent = chunk[1];
            result_f64 = executor.execute(test);
            let result: usize = int_from_f64_checked(result_f64)
                .unwrap_or_else(|| panic!("Expected SwitchWithDefault discriminant to be valid usize, found {discriminant}"));
            if result == discriminant {
                return executor.execute(consequent);
            }
        }

        executor.execute(self.default_consequent)
    }
}

impl<E, M, S, T> Executable<E, M, S, T> for JumpLoop {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        let Some(&first_branch) = self.branches.first() else {
            return 0.0;
        };

        let Some(&last_branch) = self.branches.last() else {
            return 0.0;
        };

        let mut branch_to_execute = first_branch;

        loop {
            let next_branch = executor.execute(branch_to_execute);

            if branch_to_execute == last_branch {
                return next_branch;
            }

            let Some(next_branch_index) = int_from_f64_checked::<usize>(next_branch) else {
                return 0.0;
            };

            if next_branch_index >= self.branches.len() {
                return 0.0;
            }

            branch_to_execute = self.branches[next_branch_index];

            match executor.control_mut() {
                ControlFlow::Continue(()) => {}
                ControlFlow::Break(stack) => {
                    return stack.pop().unwrap_or_default();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sonorust_memory::access::MemoryAccess;
    use sonorust_model::entity::EntityId;

    use crate::{
        Executor, IterativeInterpreter,
        node::ResolvedNode,
        opcode::{Add, Block, Break, Execute, OpCode, SideEffectAccess, Subtract, TimingAccess},
    };

    struct NoOp;

    impl MemoryAccess for NoOp {
        fn read(&self, _current_entity: EntityId, _block_id: u16, _index: usize) -> Option<f64> {
            None
        }

        fn write(&mut self, _current_entity: EntityId, _block_id: u16, _index: usize, _value: f64) {
        }
    }

    impl SideEffectAccess for NoOp {
        fn add(&mut self, _side_effect: crate::side_effect::SideEffect) {}
    }

    impl TimingAccess for NoOp {
        fn beat_to_time(&self, _beat: f64) -> f64 {
            0.0
        }

        fn beat_to_bpm(&self, _beat: f64) -> f64 {
            0.0
        }

        fn time_to_scaled_time(&self, _time: f64) -> f64 {
            0.0
        }
    }

    #[test]
    fn test_block_break() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(2.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![1, 2] })),
            ResolvedNode::OpCode(OpCode::Break(Break { count: 0, value: 1 })),
            ResolvedNode::OpCode(OpCode::Subtract(Subtract { inputs: vec![2, 1] })),
            ResolvedNode::OpCode(OpCode::Execute(Execute {
                nodes: vec![3, 4, 5],
            })),
            ResolvedNode::OpCode(OpCode::Block(Block { body: 6 })),
        ];
        let mut memory_access = NoOp;
        let mut side_effect_access = NoOp;
        let timing_access = NoOp;
        let mut interpreter = IterativeInterpreter::new(
            EntityId(0),
            &nodes,
            &mut memory_access,
            &mut side_effect_access,
            &timing_access,
        );
        let result = interpreter.execute(7);
        assert_eq!(result, 1.0);
    }
}
