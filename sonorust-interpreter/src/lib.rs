pub mod access;
pub mod node;
pub mod opcode;
pub mod side_effect;
pub(crate) mod util;

use std::ops::ControlFlow;

use access::{MemoryAccess, SideEffectAccess, TimingAccess};
use node::ResolvedNode;
use sonorust_model::entity::EntityId;

type Value = f64;
type ControlFlowState = ControlFlow<Vec<f64>>;

pub trait Executable<E, M, S, T> {
    fn execute(&self, executor: &mut E) -> Value
    where
        E: Executor<M, S, T>,
        M: MemoryAccess,
        S: SideEffectAccess,
        T: TimingAccess;
}

pub trait Executor<M: MemoryAccess, S: SideEffectAccess, T: TimingAccess>: Sized {
    fn with_control(&mut self, flow: ControlFlowState);
    fn control(&self) -> &ControlFlowState;
    fn control_mut(&mut self) -> &mut ControlFlowState;
    fn timing_access(&self) -> &T;
    fn target_entity(&self) -> EntityId;
    fn side_effect_access(&mut self) -> &mut S;
    fn memory_access(&mut self) -> &mut M;
    fn execute(&mut self, idx: usize) -> Value;
}

pub struct IterativeInterpreter<'a, M, S, T> {
    entity_id: EntityId,
    nodes: &'a [ResolvedNode],
    memory_access: &'a mut M,
    side_effect_access: &'a mut S,
    timing_access: &'a T,
    control_flow: ControlFlowState,
}

impl<'a, M, S, T> IterativeInterpreter<'a, M, S, T> {
    pub fn new(
        entity_id: EntityId,
        nodes: &'a [ResolvedNode],
        memory_access: &'a mut M,
        side_effect_access: &'a mut S,
        timing_access: &'a T,
    ) -> Self {
        Self {
            entity_id,
            nodes,
            memory_access,
            side_effect_access,
            timing_access,
            control_flow: ControlFlow::Continue(()),
        }
    }
}

impl<'a, M, S, T> Executor<M, S, T> for IterativeInterpreter<'a, M, S, T>
where
    M: MemoryAccess,
    S: SideEffectAccess,
    T: TimingAccess,
{
    fn with_control(&mut self, flow: ControlFlowState) {
        self.control_flow = flow;
    }

    fn control(&self) -> &ControlFlowState {
        &self.control_flow
    }

    fn control_mut(&mut self) -> &mut ControlFlowState {
        &mut self.control_flow
    }

    fn timing_access(&self) -> &T {
        self.timing_access
    }

    fn target_entity(&self) -> EntityId {
        self.entity_id
    }

    fn side_effect_access(&mut self) -> &mut S {
        self.side_effect_access
    }

    fn memory_access(&mut self) -> &mut M {
        self.memory_access
    }

    fn execute(&mut self, node_index: usize) -> Value {
        let node = &self.nodes[node_index];
        // match node {
        //     ResolvedNode::Value(_) => {}
        //     ResolvedNode::OpCode(opcode) => match opcode {
        //         OpCode::Abs(_) => {}
        //         OpCode::Negate(_) => {}
        //         OpCode::Add(_) => {}
        //         OpCode::Subtract(_) => {}
        //         OpCode::Multiply(_) => {}
        //         OpCode::Divide(_) => {}
        //         OpCode::Mod(_) => {}
        //         OpCode::Rem(_) => {}
        //         OpCode::Power(_) => {}
        //         OpCode::Clamp(_) => {}
        //         OpCode::Lerp(_) => {}
        //         OpCode::Unlerp(_) => {}
        //         OpCode::UnlerpClamped(_) => {}
        //         OpCode::Min(_) => {}
        //         OpCode::Max(_) => {}
        //         OpCode::Remap(_) => {}
        //         OpCode::Round(_) => {}
        //         OpCode::Floor(_) => {}
        //         OpCode::Ceil(_) => {}
        //         OpCode::Sin(_) => {}
        //         OpCode::Cos(_) => {}
        //         OpCode::Arctan2(_) => {}
        //         OpCode::EaseInCubic(_) => {}
        //         OpCode::EaseInQuad(_) => {}
        //         OpCode::EaseOutQuad(_) => {}
        //         OpCode::Equal(_) => {}
        //         OpCode::NotEqual(_) => {}
        //         OpCode::Greater(_) => {}
        //         OpCode::GreaterOr(_) => {}
        //         OpCode::Less(_) => {}
        //         OpCode::LessOr(_) => {}
        //         OpCode::And(_) => {}
        //         OpCode::Or(_) => {}
        //         OpCode::Not(_) => {}
        //         OpCode::Get(_) => {}
        //         OpCode::GetShifted(_) => {}
        //         OpCode::Set(_) => {}
        //         OpCode::SetAdd(_) => {}
        //         OpCode::SetMultiply(_) => {}
        //         OpCode::SetShifted(_) => {}
        //         OpCode::Spawn(_) => {}
        //         OpCode::Draw(_) => {}
        //         OpCode::BeatToTime(_) => {}
        //         OpCode::BeatToBPM(_) => {}
        //         OpCode::TimeToScaledTime(_) => {}
        //         OpCode::DebugLog(_) => {}
        //         OpCode::Play(_) => {}
        //         OpCode::PlayLooped(_) => {}
        //         OpCode::PlayLoopedScheduled(_) => {}
        //         OpCode::PlayScheduled(_) => {}
        //         OpCode::StopLooped(_) => {}
        //         OpCode::StopLoopedScheduled(_) => {}
        //         OpCode::HasEffectClip(_) => {}
        //         OpCode::HasParticleEffect(_) => {}
        //         OpCode::DestroyParticleEffect(_) => {}
        //         OpCode::MoveParticleEffect(_) => {}
        //         OpCode::SpawnParticleEffect(_) => {}
        //         OpCode::HasSkinSprite(_) => {}
        //         OpCode::Judge(_) => {}
        //         OpCode::ExportValue(_) => {}
        //         other => {
        //             info!("Executing {:?}", &other);
        //         }
        //     },
        // }
        node.execute(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::opcode::resolve_opcode;
    use sonorust_model::engine::play_data::Node;
    use std::collections::HashSet;

    #[test]
    fn test_all_functions_implemented() {
        let play_data_json = include_str!("../playData.json");

        let nodes: Vec<Node> = serde_json::from_str(play_data_json).unwrap();
        let failures: HashSet<_> = nodes
            .into_iter()
            .filter_map(|json_node| match json_node {
                Node::Literal { .. } => None,
                Node::FunctionCall { func, args } => {
                    resolve_opcode(func.clone(), args).is_err().then_some(func)
                }
            })
            .collect();

        for func in &failures {
            eprintln!("{func}");
        }

        if !failures.is_empty() {
            panic!("not all opcodes are implemented");
        }
    }
}
