use sonorust_model::engine::play_data::Node;

use super::*;
use crate::opcode::{OpCode, Result, errors, resolve_opcode};

#[derive(Debug)]
pub enum ResolvedNode {
    Value(f64),
    OpCode(OpCode),
}

impl<E, M, S, T> Executable<E, M, S, T> for ResolvedNode {
    fn execute(&self, executor: E) -> (E, f64)
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess,
    {
        match self {
            ResolvedNode::Value(value) => (executor, *value),
            ResolvedNode::OpCode(op_code) => op_code.execute(executor),
        }
    }
}

impl TryFrom<Node> for ResolvedNode {
    type Error = errors::Error;

    fn try_from(value: Node) -> Result<Self> {
        match value {
            Node::Literal { value } => Ok(Self::Value(value)),
            Node::FunctionCall { func, args } => Ok(Self::OpCode(resolve_opcode(func, args)?)),
        }
    }
}
