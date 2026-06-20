pub use crate::{
    Executable, Executor,
    access::{MemoryAccess, SideEffectAccess, TimingAccess},
    node::ResolvedNode,
};
use sonorust_macros::opcode_registry;

mod audio;
mod control_flow;
mod debug;
mod easing;
mod gameplay;
mod logical;
mod math;
mod memory;
mod particle;
mod replay;
mod side_effect;
mod skin;
mod timing;

pub mod errors;
pub type Result<T, E = errors::Error> = std::result::Result<T, E>;

opcode_registry! {
    // Control Flow
    Execute { ..nodes },
    If { test, consequent, alternate },
    Block { body },
    Break { count, value },
    While { test, body },
    SwitchInteger { discriminant, ..consequents },
    SwitchIntegerWithDefault { discriminant, ..consequents, default_consequent },
    SwitchWithDefault { discriminant, ..tests_and_consequents, default_consequent },
    JumpLoop { ..branches },

    // Math
    Abs { value },
    Negate { value },
    Add { ..inputs },
    Subtract { ..inputs },
    Multiply { ..inputs },
    Divide { ..inputs },
    // Mod { ..inputs }  // alias of `Rem { ..inputs }`
    Rem { ..inputs },
    Power { ..inputs },
    Clamp { min, max, value },
    Lerp { min, max, value },
    Unlerp { min, max, value },
    UnlerpClamped { min, max, value },
    Min { x, y },
    Max { x, y },
    Remap { from_min, from_max, to_min, to_max, value },
    RemapClamped { from_min, from_max, to_min, to_max, value },
    Round { value },
    Floor { value },
    Ceil { value },
    Trunc { value },
    Sin { value },
    Cos { value },
    Arctan { value },
    Arctan2 { x, y },
    Log { value },

    // Easing
    EaseInCubic { value },
    EaseOutCubic { value },
    EaseInQuad { value },
    EaseOutQuad { value },
    EaseInOutQuad { value },
    EaseOutInQuad { value },

    // Logical
    Equal { lhs, rhs },
    NotEqual { lhs, rhs },
    Greater { lhs, rhs },
    GreaterOr { lhs, rhs },
    Less { lhs, rhs },
    LessOr { lhs, rhs },
    And { ..inputs },
    Or { ..inputs },
    Not { value },

    // Memory
    Get { block_id, index },
    GetShifted { block_id, x, y, s },
    Set { block_id, index, value },
    SetAdd { block_id, index, value },
    SetMultiply { block_id, index, value },
    SetShifted { block_id, x, y, s, value },

    // Side Effects
    Spawn { archetype_id },
    Draw {
        sprite_id,
        x1,
        y1,
        x2,
        y2,
        x3,
        y3,
        x4,
        y4,
        z,
        alpha,
    },

    // Timing
    BeatToTime { beat },
    BeatToBPM { beat },
    TimeToScaledTime { time },

    // Debug
    DebugLog { value },

    // Audio
    Play { id, distance },
    PlayLooped { id },
    PlayLoopedScheduled { id, time },
    PlayScheduled { id, time, distance },
    StopLooped { id },
    StopLoopedScheduled { id, time },
    HasEffectClip { id },

    // Particle
    HasParticleEffect { id },
    DestroyParticleEffect { id },
    MoveParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4 },
    SpawnParticleEffect { id, x1, y1, x2, y2, x3, y3, x4, y4, duration, is_looped },

    // Skin
    HasSkinSprite { id },

    // Gameplay
    Judge { source, target, min_perfect, max_perfect, min_great, max_great, min_good, max_good },

    // Replay
    ExportValue { index, value },
    StreamSet { id, key, value },
}
