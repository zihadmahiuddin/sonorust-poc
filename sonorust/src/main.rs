#![feature(try_trait_v2)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    hash::Hash,
    io::{Cursor, Read},
    ops::ControlFlow,
    sync::Arc,
};

use archetype::EnginePlayDataArchetype;
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bytecode::Node;
use engine_configuration::EngineConfiguration;
use engine_play_data::{Bucket, EnginePlayData};
use flate2::read::GzDecoder;
use interpreter::memory::{
    MemoryRegion,
    blocks::{
        archetype::life::ArchetypeLife,
        engine::rom::EngineRom,
        entity::{
            data_array::{EntityData, EntityDataArray},
            despawn_array::EntityDespawn,
            info::{EntityInfo, EntityInfoArray, EntityState},
            input::EntityInputArray,
            memory::{EntityMemory, EntityMemoryArray},
            shared_memory_array::{EntitySharedMemory, EntitySharedMemoryArray},
        },
        level::{
            bucket::LevelBucket, data::LevelData, life::LevelLife, memory::LevelMemory,
            option::LevelOption, score::LevelScore,
        },
        runtime::{
            background::RuntimeBackground, environment::RuntimeEnvironment,
            particle_transform::RuntimeParticleTransform, skin_transform::RuntimeSkinTransform,
            touch::RuntimeTouchArray, ui::RuntimeUi, ui_configuration::RuntimeUiConfiguration,
            update::RuntimeUpdate,
        },
        temporary::TemporaryMemory,
    },
};
use level_data::{LevelDataEntity, LevelDataEntityDataPayload, LevelDataJson};
use level_info::LevelInfo;
use ordered_float::OrderedFloat;
use sonorust_macros::generate_memory_access;

mod archetype;
mod bytecode;
mod engine_configuration;
mod engine_play_data;
mod interpreter;
mod level_data;
mod level_info;

fn main() {
    let server = "http://192.168.0.100:8080";
    let level = "dev";

    // let server = "https://sonolus.sekai.best";
    // let level = "sekai-best-429-1416-expert";

    let level_info = minreq::get(format!("{server}/sonolus/levels/{level}"))
        .send()
        .unwrap()
        .json::<LevelInfo>()
        .unwrap();

    let engine_play_data = minreq::get(format!(
        "{}{}",
        server, &level_info.item.engine.play_data.url
    ))
    .send()
    .unwrap()
    .into_bytes();

    let mut gzip_decoder = GzDecoder::new(Cursor::new(engine_play_data));
    let mut json_str = String::new();
    gzip_decoder.read_to_string(&mut json_str).unwrap();
    let engine_play_data = serde_json::from_str::<EnginePlayData>(&json_str).unwrap();

    let engine_configuration = minreq::get(format!(
        "{}{}",
        server, &level_info.item.engine.configuration.url
    ))
    .send()
    .unwrap()
    .into_bytes();

    let mut gzip_decoder = GzDecoder::new(Cursor::new(engine_configuration));
    let mut json_str = String::new();
    gzip_decoder.read_to_string(&mut json_str).unwrap();
    let engine_configuration = serde_json::from_str::<EngineConfiguration>(&json_str).unwrap();

    let level_data = minreq::get(format!("{}{}", server, &level_info.item.data.url))
        .send()
        .unwrap()
        .into_bytes();

    let mut gzip_decoder = GzDecoder::new(Cursor::new(level_data));
    let mut json_str = String::new();
    gzip_decoder.read_to_string(&mut json_str).unwrap();
    let level_data = serde_json::from_str::<LevelDataJson>(&json_str).unwrap();

    let bgm_bytes = minreq::get(format!("{}{}", server, &level_info.item.bgm.url))
        // let bgm_bytes = minreq::get(&level_info.item.bgm.url)
        .send()
        .unwrap()
        .into_bytes();

    let sonorust_plugin = SonorustPlugin::new(engine_play_data, engine_configuration, level_data);

    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(sonorust_plugin)
        .add_systems(
            Startup,
            (preparation, preprocessing, spawn_ordering).chain(),
        )
        .add_systems(
            Update,
            (
                set_runtime_update_values,
                (should_spawn_callback, spawning).chain(),
                initialization,
                sequential_update,
                input,
                parallel_update,
                (despawning, terminate_callback).chain(),
                presentation,
            ),
        )
        .insert_resource(LevelBgmBytes(bgm_bytes))
        .run();
}

generate_memory_access!(
    [
        RuntimeEnvironment,
        RuntimeUpdate,
        RuntimeTouchArray,
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        RuntimeUi,
        RuntimeUiConfiguration,
        LevelMemory,
        LevelData,
        LevelOption,
        LevelBucket,
        LevelScore,
        LevelLife,
        EngineRom,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntityDataArray,
        EntitySharedMemoryArray,
        EntityInfoArray,
        ArchetypeLife,
        TemporaryMemory,
    ]

    Preprocess: [
        RuntimeEnvironment,
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        RuntimeUi,
        RuntimeUiConfiguration,
        LevelMemory,
        LevelData,
        LevelBucket,
        LevelScore,
        LevelLife,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntityDataArray,
        EntitySharedMemoryArray,
        ArchetypeLife,
        TemporaryMemory,
    ]
    SpawnOrder: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    ShouldSpawn: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    Initialize: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    UpdateSequential: [
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        LevelMemory,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntitySharedMemoryArray,
        TemporaryMemory,
    ]
    Touch: [
        RuntimeSkinTransform,
        RuntimeParticleTransform,
        RuntimeBackground,
        LevelMemory,
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        EntitySharedMemoryArray,
        TemporaryMemory,
    ]
    UpdateParallel: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
    Terminate: [
        EntityMemoryArray,
        EntityDespawn,
        EntityInputArray,
        TemporaryMemory,
    ]
);

#[derive(Resource)]
struct LevelBgmBytes(Vec<u8>);

#[derive(Resource)]
struct LevelBgm(Handle<AudioInstance>);

#[derive(Deref, DerefMut, Resource)]
struct EntityMap(BTreeMap<EntityId, Entity>);

#[derive(Default, Deref, DerefMut, Resource)]
struct SpawnQueue(BTreeMap<SpawnOrder, BTreeSet<EntityId>>);

#[derive(Default, Deref, DerefMut, Resource)]
struct InitializeQueue(BTreeSet<EntityId>);

#[derive(Deref, Event)]
struct ShouldSpawnEvent(EntityId);

fn preparation(
    mut commands: Commands,
    level_bgm_bytes: Res<LevelBgmBytes>,
    mut audio_sources: ResMut<Assets<AudioSource>>,
    audio: Res<Audio>,
) {
    commands.spawn(Camera2d);
    let audio_source = audio_sources.add(AudioSource {
        sound: StaticSoundData::from_cursor(Cursor::new(level_bgm_bytes.0.clone())).unwrap(),
    });
    let instance_handle = audio.play(audio_source).with_volume(0.3).handle();
    commands.insert_resource(LevelBgm(instance_handle));
}

#[derive(SystemParam)]
struct Interpreter<'w> {
    pub block_stack: ResMut<'w, InterpreterBlockStack>,
    pub nodes: Res<'w, InterpreterNodes>,
}

impl<'w> Interpreter<'w> {
    pub fn interpret<M: MemoryAccess>(
        &mut self,
        context: &mut InterpreterContext<M>,
        index: usize,
    ) -> f64 {
        match self.interpret_inner(context, index) {
            ControlFlow::Continue(v) => v,
            ControlFlow::Break(_) => {
                // Should not reach top-level with a Break
                println!("Warning: Break bubbled to top level");
                0.0
            }
        }
    }

    fn interpret_inner<M: MemoryAccess>(
        &mut self,
        context: &mut InterpreterContext<M>,
        index: usize,
    ) -> ControlFlow<usize, f64> {
        let node = &self.nodes[index].clone();

        match node {
            Node::Literal { value } => return ControlFlow::Continue(*value),
            Node::FunctionCall { func, args } => match func.as_str() {
                "Add" => {
                    let mut acc = self.interpret_inner(context, args[0])?;
                    for &arg in &args[1..] {
                        acc += self.interpret_inner(context, arg)?;
                    }
                    return ControlFlow::Continue(acc);
                }

                "Subtract" => {
                    let mut acc = self.interpret_inner(context, args[0])?;
                    for &arg in &args[1..] {
                        acc -= self.interpret_inner(context, arg)?;
                    }
                    return ControlFlow::Continue(acc);
                }

                "Multiply" => {
                    let mut acc = self.interpret_inner(context, args[0])?;
                    for &arg in &args[1..] {
                        acc *= self.interpret_inner(context, arg)?;
                    }
                    return ControlFlow::Continue(acc);
                }

                "Divide" => {
                    let mut acc = self.interpret_inner(context, args[0])?;
                    for &arg in &args[1..] {
                        acc /= self.interpret_inner(context, arg)?;
                    }
                    return ControlFlow::Continue(acc);
                }

                "Negate" => {
                    return ControlFlow::Continue(-self.interpret_inner(context, args[0])?);
                }
                "Not" => {
                    let value = -self.interpret_inner(context, args[0])?;
                    if value == 0.0 {
                        return ControlFlow::Continue(1.0);
                    } else {
                        return ControlFlow::Continue(0.0);
                    }
                }

                "And" => {
                    let mut result = self.interpret_inner(context, args[0])? as i64;
                    for &arg in &args[1..] {
                        let val = self.interpret_inner(context, arg)? as i64;
                        if val == 0 {
                            return ControlFlow::Continue(0.0);
                        }
                        result &= val;
                    }
                    return ControlFlow::Continue(result as f64);
                }

                "Execute" => {
                    let mut last = 0.0;
                    for &arg in args {
                        last = self.interpret_inner(context, arg)?;
                    }
                    return ControlFlow::Continue(last);
                }
                "Get" => {
                    let block_id = self.interpret_inner(context, args[0])?.floor() as u16;
                    let index = self.interpret_inner(context, args[1])?.floor() as usize;
                    match context
                        .memory_access
                        .read(context.current_entity, block_id, index)
                    {
                        Some(result) => return ControlFlow::Continue(result),
                        None => {
                            println!("Get value in block {block_id}, index {index}");
                            return ControlFlow::Continue(0.0);
                        }
                    }
                }
                "Set" => {
                    let block_id = self.interpret_inner(context, args[0])? as u16;
                    let index = self.interpret_inner(context, args[1])? as usize;
                    let value = self.interpret_inner(context, args[2])?;
                    context
                        .memory_access
                        .write(context.current_entity, block_id, index, value);
                    return ControlFlow::Continue(value);
                }
                "SetAdd" => {
                    // Extract args
                    let block_id = self.interpret_inner(context, args[0])?.floor() as u16;
                    let index = self.interpret_inner(context, args[1])?.floor() as usize;
                    let value = self.interpret_inner(context, args[2])?;

                    // Read current value from memory
                    let current = context
                        .memory_access
                        .read(context.current_entity, block_id, index)
                        .unwrap_or(0.0);

                    // Calculate new value
                    let new_value = current + value;

                    // Write new value back to memory
                    context
                        .memory_access
                        .write(context.current_entity, block_id, index, new_value);

                    return ControlFlow::Continue(new_value);
                }
                "BeatToTime" => {
                    let beat = self.interpret_inner(context, args[0])?;
                    return ControlFlow::Continue(context.bpm_changes.beat_to_time(beat));
                }
                "BeatToBPM" => {
                    let beat = self.interpret_inner(context, args[0])?;
                    return ControlFlow::Continue(context.bpm_changes.beat_to_bpm(beat));
                }
                "TimeToScaledTime" => {
                    let time = self.interpret_inner(context, args[0])?;
                    return ControlFlow::Continue(
                        context.time_scale_changes.time_to_scaled_time(time),
                    );
                }
                "DebugLog" => {
                    let value = self.interpret_inner(context, args[0])?;
                    println!("Debug Log (by {}): {value}", *context.current_entity,);
                    return ControlFlow::Continue(0.0);
                }
                "Equal" => {
                    let lhs = self.interpret_inner(context, args[0])?;
                    let rhs = self.interpret_inner(context, args[1])?;
                    let result = lhs == rhs;
                    if lhs > 540.0 {
                        // dbg!(lhs, rhs, result);
                    }
                    if result {
                        return ControlFlow::Continue(1.0);
                    } else {
                        return ControlFlow::Continue(0.0);
                    }
                }
                "Less" => {
                    let lhs = self.interpret_inner(context, args[0])?;
                    let rhs = self.interpret_inner(context, args[1])?;
                    if lhs < rhs {
                        return ControlFlow::Continue(1.0);
                    } else {
                        return ControlFlow::Continue(0.0);
                    }
                }
                "Greater" => {
                    let lhs = self.interpret_inner(context, args[0])?;
                    let rhs = self.interpret_inner(context, args[1])?;
                    if lhs > rhs {
                        return ControlFlow::Continue(1.0);
                    } else {
                        return ControlFlow::Continue(0.0);
                    }
                }
                "GreaterOr" => {
                    let lhs = self.interpret_inner(context, args[0])?;
                    let rhs = self.interpret_inner(context, args[1])?;
                    if lhs >= rhs {
                        return ControlFlow::Continue(1.0);
                    } else {
                        return ControlFlow::Continue(0.0);
                    }
                }
                "Block" => {
                    self.block_stack.0 += 1; // Enter block

                    let body_idx = args[0];
                    let result = match self.interpret_inner(context, body_idx) {
                        ControlFlow::Break(mut n) => {
                            self.block_stack.0 -= 1; // Leave this block

                            n -= 1;
                            if n == 0 {
                                ControlFlow::Continue(0.0) // Break finished
                            } else {
                                ControlFlow::Break(n) // Bubble up
                            }
                        }
                        other => {
                            self.block_stack.0 -= 1; // Leave this block
                            other
                        }
                    };

                    return result;
                }
                "While" => {
                    let cond_idx = args[0];
                    let body_idx = args[1];

                    let mut result = 0.0;

                    self.block_stack.0 += 1; // Enter loop block

                    loop {
                        match self.interpret_inner(context, cond_idx) {
                            ControlFlow::Continue(0.0) => {
                                break; // condition false, exit loop
                            }
                            ControlFlow::Continue(_) => {
                                match self.interpret_inner(context, body_idx) {
                                    ControlFlow::Continue(val) => {
                                        result = val; // remember last body result
                                    }
                                    ControlFlow::Break(mut n) => {
                                        self.block_stack.0 -= 1; // exit this block
                                        n -= 1;

                                        if n == 0 {
                                            break; // our break — exit loop
                                        } else {
                                            return ControlFlow::Break(n); // bubble up
                                        }
                                    }
                                }
                            }
                            ControlFlow::Break(n) => {
                                self.block_stack.0 -= 1;
                                return ControlFlow::Break(n); // unusual case — breaking inside condition
                            }
                        }
                    }

                    self.block_stack.0 -= 1;
                    return ControlFlow::Continue(result);
                }
                "If" => {
                    let cond = match self.interpret_inner(context, args[0]) {
                        ControlFlow::Continue(v) => v,
                        c @ ControlFlow::Break(_) => return c, // propagate breaks
                    };

                    return if cond != 0.0 {
                        self.interpret_inner(context, args[1])
                    } else {
                        self.interpret_inner(context, args[2])
                    };
                }
                "Break" => {
                    let count = self.interpret_inner(context, args[0])?.floor() as usize;
                    return ControlFlow::Break(count);
                }
                "Spawn" => {
                    // let archetype_index = self.self.interpret_inner(context, args[0])?.floor() as usize;
                    // dbg!(args);
                    // println!("Sadly skipping spawn for now ;-;");
                    return ControlFlow::Continue(0.0);
                }
                "Draw" => {
                    // let sprite_id = self.self.interpret_inner(context, args[0].floor()? as usize).floor() as usize;
                    // let x1 = self.self.interpret_inner(context, args[1].floor()? as usize).floor() as usize;
                    // let y1 = self.self.interpret_inner(context, args[2].floor()? as usize).floor() as usize;
                    // let x2 = self.self.interpret_inner(context, args[3].floor()? as usize).floor() as usize;
                    // let y2 = self.self.interpret_inner(context, args[4].floor()? as usize).floor() as usize;
                    // let x3 = self.self.interpret_inner(context, args[5].floor()? as usize).floor() as usize;
                    // let y3 = self.self.interpret_inner(context, args[6].floor()? as usize).floor() as usize;
                    // let x4 = self.self.interpret_inner(context, args[7].floor()? as usize).floor() as usize;
                    // let y4 = self.self.interpret_inner(context, args[8].floor()? as usize).floor() as usize;
                    // let z = self.self.interpret_inner(context, args[9].floor()? as usize).floor() as usize;
                    // let alpha = self.self.interpret_inner(context, args[10].floor()? as usize).floor() as usize;
                    // todo!("DRAW!!!");
                    return ControlFlow::Continue(0.0);
                }
                func => {
                    println!("Unimplemented function {func}");
                }
            },
        }

        ControlFlow::Continue(0.0)
    }
}

fn preprocessing(
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    entities: Res<EntityMap>,
    mut memory: PreprocessMemoryAccess,
    mut interpreter: Interpreter,
) {
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.iter() {
        let Some(preprocess_callback) = &entity.archetype.preprocess.as_ref() else {
            continue;
        };
        let order = preprocess_callback.order.unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeSet<_>| {
                entities.insert(*entity_id);
            })
            .or_insert(BTreeSet::from([*entity_id]));
    }

    for (_order, entity_ids) in callback_order_map {
        for entity_id in entity_ids.iter() {
            let entity = &entities[entity_id];
            let Some(preprocess) = &entity.archetype.preprocess else {
                continue;
            };

            let mut context = InterpreterContext {
                bpm_changes: &bpm_changes,
                time_scale_changes: &time_scale_changes,
                current_entity: entity.id,
                memory_access: &mut memory,
            };

            interpreter.interpret(&mut context, preprocess.index);
        }
    }
}

fn spawn_ordering(
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    mut interpreter: Interpreter,
    entities: Res<EntityMap>,
    mut spawn_queue: ResMut<SpawnQueue>,
    mut memory: SpawnOrderMemoryAccess,
) {
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.iter() {
        let spawn_order_callback = &entity.archetype.spawn_order.as_ref();
        let order = spawn_order_callback
            .and_then(|callback| callback.order)
            .unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeSet<_>| {
                entities.insert(*entity_id);
            })
            .or_insert(BTreeSet::from([*entity_id]));
    }

    for (_, entity_ids) in callback_order_map {
        for entity_id in entity_ids.iter() {
            let entity = &entities[entity_id];
            let order = match &entity.archetype.spawn_order {
                Some(spawn_order) => {
                    let mut context = InterpreterContext {
                        bpm_changes: &bpm_changes,
                        time_scale_changes: &time_scale_changes,
                        current_entity: entity.id,
                        memory_access: &mut memory,
                    };
                    interpreter.interpret(&mut context, spawn_order.index)
                }
                _ => 0.0,
            };

            spawn_queue
                .entry(order.into())
                .and_modify(|items| {
                    items.insert(entity.id);
                })
                .or_insert_with(|| BTreeSet::from([entity.id]));
        }
    }
}

fn set_runtime_update_values(
    time_scale_changes: Res<TimeScaleChanges>,
    time: Res<Time>,
    level_bgm: Res<LevelBgm>,
    audio_instances: Res<Assets<AudioInstance>>,
    mut runtime_update: ResMut<RuntimeUpdate>,
) {
    let Some(level_bgm_audio) = audio_instances.get(&level_bgm.0) else {
        return;
    };

    let delta_time = time.delta_secs_f64();
    let time = level_bgm_audio.state().position().unwrap_or_default();
    let scaled_time = time_scale_changes.time_to_scaled_time(time);

    let touch_count = 0.0; // TODO
    runtime_update.time = time;
    runtime_update.delta_time = delta_time;
    runtime_update.scaled_time = scaled_time;
    runtime_update.touch_count = touch_count;
}

fn should_spawn_callback(
    entities: Res<EntityMap>,
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    mut spawn_queue: ResMut<SpawnQueue>,
    mut interpreter: Interpreter,
    mut memory: ShouldSpawnMemoryAccess,
    mut should_spawn_events: EventWriter<ShouldSpawnEvent>,
) {
    let mut orders_to_remove = Vec::new();

    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.iter() {
        let should_spawn_callback = &entity.archetype.should_spawn.as_ref();
        let order = should_spawn_callback
            .and_then(|callback| callback.order)
            .unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeSet<_>| {
                entities.insert(*entity_id);
            })
            .or_insert(BTreeSet::from([*entity_id]));
    }

    let mut should_spawn_map = BTreeSet::new();
    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities {
            let entity = &entities[&entity_id];
            let should_spawn = if let Some(should_spawn) = &entity.archetype.should_spawn {
                let mut context = InterpreterContext {
                    bpm_changes: &bpm_changes,
                    time_scale_changes: &time_scale_changes,
                    current_entity: entity_id,
                    memory_access: &mut memory,
                };

                interpreter.interpret(&mut context, should_spawn.index) != 0.0
            } else {
                true
            };
            if should_spawn {
                should_spawn_map.insert(entity_id);
            }
        }
    }

    for (order, entity_ids) in spawn_queue.iter_mut() {
        let entity_ids_clone = entity_ids.clone();
        for entity_id in entity_ids_clone.into_iter() {
            if should_spawn_map.contains(&entity_id) {
                should_spawn_events.write(ShouldSpawnEvent(entity_id));
                entity_ids.remove(&entity_id);
            }
        }

        if entity_ids.is_empty() {
            orders_to_remove.push(*order);
        }
    }

    for order in orders_to_remove {
        spawn_queue.remove(&order);
    }
}

fn spawning(
    mut commands: Commands,
    entities: Res<EntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut initialize_queue: ResMut<InitializeQueue>,
    mut should_spawn_events: EventReader<ShouldSpawnEvent>,
    mut entity_info_array: ResMut<EntityInfoArray>,
) {
    initialize_queue.clear();

    for ShouldSpawnEvent(entity_id) in should_spawn_events.read() {
        initialize_queue.insert(*entity_id);
        if let Some(entity_info) = entity_info_array.entry_mut(entity_id) {
            entity_info.state = EntityState::Active;
        }

        let entity = &entities[entity_id];
        println!("Spawn entity {} ({})", **entity_id, entity.archetype.name);
        let color = Color::hsl(360. * **entity_id as f32 / entities.len() as f32, 0.95, 0.7);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}

fn initialization(
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    initialize_queue: Res<InitializeQueue>,
    entities: Res<EntityMap>,
    mut interpreter: Interpreter,
    mut memory: InitializeMemoryAccess,
) {
    let mut callback_order_map = BTreeMap::new();
    for entity_id in initialize_queue.iter() {
        let entity = &entities[entity_id];
        if let Some(initialize_callback) = &entity.archetype.initialize {
            let order = initialize_callback.order.unwrap_or_default();
            callback_order_map
                .entry(order)
                .and_modify(|entities: &mut BTreeSet<_>| {
                    entities.insert(*entity_id);
                })
                .or_insert(BTreeSet::from([*entity_id]));
        }
    }

    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities.iter() {
            let entity = &entities[entity_id];
            let Some(initialize) = &entity.archetype.initialize else {
                continue;
            };
            let initialize_index = initialize.index;

            let mut context = InterpreterContext {
                bpm_changes: &bpm_changes,
                time_scale_changes: &time_scale_changes,
                current_entity: *entity_id,
                memory_access: &mut memory,
            };

            interpreter.interpret(&mut context, initialize_index);
        }
    }
}

fn sequential_update(
    entities: Res<EntityMap>,
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    mut interpreter: Interpreter,
    mut memory: UpdateSequentialMemoryAccess,
) {
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.iter() {
        if memory.entity_info_array.entry(entity_id).unwrap().state != EntityState::Active {
            continue;
        }

        if let Some(update_sequential_callback) = &entity.archetype.update_sequential {
            let order = update_sequential_callback.order.unwrap_or_default();
            callback_order_map
                .entry(order)
                .and_modify(|entities: &mut BTreeSet<_>| {
                    entities.insert(*entity_id);
                })
                .or_insert(BTreeSet::from([*entity_id]));
        }
    }

    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities.iter() {
            let entity = &entities[entity_id];
            let Some(update_sequential) = &entity.archetype.update_sequential else {
                continue;
            };
            let update_sequential_index = update_sequential.index;

            let mut context = InterpreterContext {
                bpm_changes: &bpm_changes,
                time_scale_changes: &time_scale_changes,
                current_entity: *entity_id,
                memory_access: &mut memory,
            };

            interpreter.interpret(&mut context, update_sequential_index);
        }
    }
}

fn input() {}

fn parallel_update(
    entities: Res<EntityMap>,
    bpm_changes: Res<BpmChanges>,
    time_scale_changes: Res<TimeScaleChanges>,
    mut interpreter: Interpreter,
    mut memory: UpdateParallelMemoryAccess,
) {
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.iter() {
        if memory.read(*entity_id, EntityInfo::ID, 2) != Some(1.0) {
            continue;
        }

        if let Some(update_parallel_callback) = &entity.archetype.update_parallel {
            let order = update_parallel_callback.order.unwrap_or_default();
            callback_order_map
                .entry(order)
                .and_modify(|entities: &mut BTreeSet<_>| {
                    entities.insert(*entity_id);
                })
                .or_insert(BTreeSet::from([*entity_id]));
        }
    }

    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities.iter() {
            let entity = &entities[entity_id];
            let Some(update_parallel) = &entity.archetype.update_parallel else {
                continue;
            };
            let update_parallel_index = update_parallel.index;

            let mut context = InterpreterContext {
                bpm_changes: &bpm_changes,
                time_scale_changes: &time_scale_changes,
                current_entity: *entity_id,
                memory_access: &mut memory,
            };

            interpreter.interpret(&mut context, update_parallel_index);
        }
    }
}

fn despawning(
    mut entity_despawn: ResMut<EntityDespawn>,
    mut entity_info_array: ResMut<EntityInfoArray>,
) {
    let despawned_entities = entity_despawn.items_clone();
    for despawned_entity_id in despawned_entities {
        let entity_info = entity_info_array.entry_mut(&despawned_entity_id).unwrap();
        if entity_info.state == EntityState::Despawned {
            continue;
        }

        entity_info.state = EntityState::Despawned;
        entity_despawn.remove(&despawned_entity_id);
    }
}

fn terminate_callback(
    bpm_changes: Res<BpmChanges>,
    entities: Res<EntityMap>,
    time_scale_changes: Res<TimeScaleChanges>,
    mut interpreter: Interpreter,
    mut memory: TerminateMemoryAccess,
) {
    let mut callback_order_map = BTreeMap::new();
    for despawned_entity_id in memory.entity_despawn.iter() {
        let entity = &entities[despawned_entity_id];
        let terminate_callback = &entity.archetype.terminate.as_ref();
        let order = terminate_callback
            .and_then(|callback| callback.order)
            .unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeSet<_>| {
                entities.insert(*despawned_entity_id);
            })
            .or_insert(BTreeSet::from([*despawned_entity_id]));
    }

    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities.iter() {
            let entity = &entities[entity_id];
            let Some(terminate) = &entity.archetype.terminate else {
                continue;
            };
            let update_parallel_index = terminate.index;

            let mut context = InterpreterContext {
                bpm_changes: &bpm_changes,
                time_scale_changes: &time_scale_changes,
                current_entity: *entity_id,
                memory_access: &mut memory,
            };
            interpreter.interpret(&mut context, update_parallel_index);
        }
    }
}

fn presentation() {}

struct BpmChange {
    bpm: f64,
    starting_beat: f64,
    starting_time: f64,
}

struct TimeScaleChange {
    time_scale: f64,
    starting_scaled_time: f64,
    starting_time: f64,
}

pub struct Entity {
    id: EntityId,
    // name: Option<String>,
    archetype_index: usize,
    archetype: Arc<EnginePlayDataArchetype>,
    data: EntityData,
}

enum SpecialArchetype {
    BpmChange { beat: f64, bpm: f64 },
    TimescaleChange { beat: f64, timescale: f64 },
}

impl From<LevelDataEntity> for Option<SpecialArchetype> {
    fn from(value: LevelDataEntity) -> Self {
        let mut data = value
            .data
            .into_iter()
            .map(|data| (data.name, data.payload))
            .collect::<HashMap<_, _>>();
        let beat = match data.remove("#BEAT")?? {
            LevelDataEntityDataPayload::Value { value } => value,
            _ => return None,
        };
        match value.archetype.as_str() {
            "#BPM_CHANGE" => {
                let bpm = match data.remove("#BPM")?? {
                    LevelDataEntityDataPayload::Value { value } => value,
                    _ => return None,
                };
                Some(SpecialArchetype::BpmChange { beat, bpm })
            }
            "#TIMESCALE_CHANGE" => {
                let timescale = match data.remove("#TIMESCALE")?? {
                    LevelDataEntityDataPayload::Value { value } => value,
                    _ => return None,
                };
                Some(SpecialArchetype::TimescaleChange { beat, timescale })
            }
            _ => None,
        }
    }
}

#[derive(Resource)]
struct BpmChanges {
    changes: Vec<BpmChange>,
}

impl BpmChanges {
    fn beat_to_last_change(&self, beat: f64) -> Option<&BpmChange> {
        self.changes
            .iter()
            .rev()
            .find(|change| change.starting_beat < beat)
    }

    pub fn beat_to_time(&self, beat: f64) -> f64 {
        let Some(bpm_change) = self.beat_to_last_change(beat) else {
            return 0.0;
        };

        let mut time = bpm_change.starting_time;

        let remaining_beats = beat - bpm_change.starting_beat;
        let bps = bpm_change.bpm / 60.0;
        let remaining_seconds = remaining_beats / bps;
        time += remaining_seconds;

        time
    }

    pub fn beat_to_bpm(&self, beat: f64) -> f64 {
        let Some(bpm_change) = self.beat_to_last_change(beat) else {
            return self
                .changes
                .first()
                .map(|change| change.bpm)
                .unwrap_or_default();
        };
        bpm_change.bpm
    }
}

#[derive(Resource)]
struct TimeScaleChanges {
    changes: Vec<TimeScaleChange>,
}

impl TimeScaleChanges {
    pub fn time_to_scaled_time(&self, time: f64) -> f64 {
        let Some(last_change) = self
            .changes
            .iter()
            .rev()
            .find(|change| change.starting_time <= time)
        else {
            return 0.0;
        };

        let delta = time - last_change.starting_time;
        last_change.starting_scaled_time + delta * last_change.time_scale
    }
}

struct InterpreterContext<'w, M: MemoryAccess> {
    bpm_changes: &'w BpmChanges,
    time_scale_changes: &'w TimeScaleChanges,
    memory_access: &'w mut M,
    current_entity: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref)]
pub struct EntityId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
struct SpawnOrder(OrderedFloat<f64>);

impl From<f64> for SpawnOrder {
    fn from(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
}

struct SonorustPlugin {
    engine_play_data: EnginePlayData,
    engine_configuration: EngineConfiguration,
    level_data: LevelDataJson,
}

impl SonorustPlugin {
    pub fn new(
        engine_play_data: EnginePlayData,
        engine_configuration: EngineConfiguration,
        level_data: LevelDataJson,
    ) -> Self {
        Self {
            engine_play_data,
            engine_configuration,
            level_data,
        }
    }

    pub fn add_memories(
        app: &mut App,
        archetypes: &HashMap<String, (usize, Arc<EnginePlayDataArchetype>)>,
        entities: &BTreeMap<EntityId, Entity>,
        buckets: &[Bucket],
        engine_configuration: &EngineConfiguration,
    ) {
        let runtime_environment = RuntimeEnvironment {
            debug_mode: false,
            screen_aspect_ratio: 1.667,
            audio_offset: 0.0,
            input_offset: 0.0,
            multiplayer: false,
        };
        let runtime_update = RuntimeUpdate::default();
        let runtime_touch_array = RuntimeTouchArray::default();
        let runtime_skin_transform = RuntimeSkinTransform::default();
        let runtime_particle_transform = RuntimeParticleTransform::default();
        let runtime_background = RuntimeBackground::default();
        let runtime_ui = RuntimeUi::default();
        let runtime_ui_configuration = RuntimeUiConfiguration::default();

        let level_memory = LevelMemory::default();
        let level_data = LevelData::default();
        let level_option = LevelOption::new(&engine_configuration.options);
        let level_bucket = LevelBucket::new(buckets.len());
        let level_score = LevelScore::default();
        let level_life = LevelLife::default();

        let engine_rom = EngineRom::default();

        let entity_memory_array = EntityMemoryArray::new(entities);
        let entity_data_array = EntityDataArray::new(entities);
        let entity_shared_memory_array = EntitySharedMemoryArray::new(entities.len());
        let entity_info_array = EntityInfoArray::new(entities.iter());
        let entity_despawn = EntityDespawn::default();
        let entity_input_array = EntityInputArray::new(entities.keys());

        let archetype_life = ArchetypeLife::new(archetypes.len());

        let temporary_memory = TemporaryMemory::default();

        app.insert_resource(runtime_environment)
            .insert_resource(runtime_update)
            .insert_resource(runtime_touch_array)
            .insert_resource(runtime_skin_transform)
            .insert_resource(runtime_particle_transform)
            .insert_resource(runtime_background)
            .insert_resource(runtime_ui)
            .insert_resource(runtime_ui_configuration)
            .insert_resource(level_memory)
            .insert_resource(level_data)
            .insert_resource(level_option)
            .insert_resource(level_bucket)
            .insert_resource(level_score)
            .insert_resource(level_life)
            .insert_resource(engine_rom)
            .insert_resource(entity_memory_array)
            .insert_resource(entity_data_array)
            .insert_resource(entity_shared_memory_array)
            .insert_resource(entity_info_array)
            .insert_resource(entity_despawn)
            .insert_resource(entity_input_array)
            .insert_resource(archetype_life)
            .insert_resource(temporary_memory);
    }
}

impl Plugin for SonorustPlugin {
    fn build(&self, app: &mut App) {
        let archetypes = self
            .engine_play_data
            .archetypes
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, archetype)| {
                (archetype.name.clone(), (index, Arc::new(archetype.clone())))
            })
            .collect::<HashMap<_, _>>();

        let (entities, mut bpm_changes, mut time_scale_changes) =
            self.level_data.entities.clone().into_iter().enumerate().fold(
                (BTreeMap::new(), Vec::new(), Vec::new()),
                |(mut entities, mut bpm_changes, mut timescale_changes), (entity_index, entity)| {
                    if let Some((archetype_index, archetype)) = archetypes.get(&entity.archetype) {
                        let mut entity_data = [0.0; EntityData::SIZE];
                        let entity_data_map = entity
                            .data
                            .iter()
                            .map(|a| (&a.name, &a.payload))
                            .collect::<HashMap<_, _>>();

                        for import in &archetype.imports {
                            if let Some(Some(payload)) = entity_data_map.get(&import.name) {
                                match payload {
                                    LevelDataEntityDataPayload::Reference { reference } => {
                                        println!(
                                            "Level Data Entity Data Payload by reference {reference}"
                                        );
                                    }
                                    LevelDataEntityDataPayload::Value { value } => {
                                        entity_data[import.index] = *value;
                                    }
                                }
                            }
                        }

                        let entity_id = EntityId(entity_index);
                        entities.insert(entity_id,Entity {
                            // name: entity.name,
                            id: entity_id,
                            archetype_index: *archetype_index,
                            archetype: Arc::clone(archetype),
                            data: EntityData::new(entity_data),
                        });
                    } else if let Some(special_archetype) = Option::<SpecialArchetype>::from(entity)
                    {
                        match special_archetype {
                            SpecialArchetype::BpmChange { beat, bpm } => {
                                bpm_changes.push((beat, bpm));
                            }
                            SpecialArchetype::TimescaleChange { beat, timescale } => {
                                timescale_changes.push((beat, timescale));
                            }
                        }
                    } else {
                        unreachable!()
                    }

                    (entities, bpm_changes, timescale_changes)
                },
            );

        bpm_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let bpm_changes =
            bpm_changes
                .into_iter()
                .fold(Vec::new(), |mut bpm_changes, (beat, bpm)| {
                    let Some(last_change) = bpm_changes.last() else {
                        bpm_changes.push(BpmChange {
                            bpm,
                            starting_beat: beat,
                            starting_time: 0.0,
                        });
                        return bpm_changes;
                    };

                    let last_change_beat_count = beat - last_change.starting_beat;
                    let last_change_bps = last_change.bpm / 60.0;
                    let last_change_duration_secs = last_change_beat_count / last_change_bps;

                    let change = BpmChange {
                        bpm,
                        starting_beat: beat,
                        starting_time: last_change.starting_time + last_change_duration_secs,
                    };
                    bpm_changes.push(change);
                    bpm_changes
                });
        let bpm_changes = BpmChanges {
            changes: bpm_changes,
        };

        time_scale_changes
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        // set default time scale from the beginning of the level to 1
        if time_scale_changes.is_empty() || time_scale_changes[0].0 != 0.0 {
            time_scale_changes.insert(0, (0.0, 1.0));
        }

        let time_scale_changes = time_scale_changes.into_iter().fold(
            Vec::new(),
            |mut time_scale_changes, (beat, time_scale)| {
                let Some(last_change) = time_scale_changes.last() else {
                    time_scale_changes.push(TimeScaleChange {
                        time_scale,
                        starting_time: bpm_changes.beat_to_time(beat),
                        starting_scaled_time: bpm_changes.beat_to_time(beat) * time_scale,
                    });
                    return time_scale_changes;
                };

                let starting_time = bpm_changes.beat_to_time(beat);
                let duration = starting_time - last_change.starting_time;
                let scaled_duration = duration * last_change.time_scale;
                let starting_scaled_time = last_change.starting_scaled_time + scaled_duration;

                let change = TimeScaleChange {
                    time_scale,
                    starting_time,
                    starting_scaled_time,
                };
                time_scale_changes.push(change);
                time_scale_changes
            },
        );
        let time_scale_changes = TimeScaleChanges {
            changes: time_scale_changes,
        };

        let interpreter_block_stack = InterpreterBlockStack::default();
        let interpreter_nodes = InterpreterNodes(self.engine_play_data.nodes.clone());

        Self::add_memories(
            app,
            &archetypes,
            &entities,
            &self.engine_play_data.buckets,
            &self.engine_configuration,
        );

        app.insert_resource(EntityMap(entities))
            .insert_resource(interpreter_block_stack)
            .insert_resource(interpreter_nodes)
            .insert_resource(bpm_changes)
            .insert_resource(time_scale_changes)
            .insert_resource(SpawnQueue::default())
            .insert_resource(InitializeQueue::default())
            .add_event::<ShouldSpawnEvent>();
    }
}

#[derive(Deref, Resource)]
struct InterpreterNodes(Vec<Node>);

#[derive(Deref, DerefMut, Default, Resource)]
struct InterpreterBlockStack(usize);

pub trait MemoryAccess {
    fn read(&self, current_entity: EntityId, block_id: u16, index: usize) -> Option<f64>;
    fn write(&mut self, current_entity: EntityId, block_id: u16, index: usize, value: f64);
}

pub fn resolve_entity_index(block_id: u16, current_entity: EntityId, index: usize) -> (u16, usize) {
    match block_id {
        EntityData::ID => (EntityDataArray::ID, *current_entity * EntityData::SIZE),
        EntitySharedMemory::ID => (
            EntitySharedMemoryArray::ID,
            *current_entity * EntitySharedMemory::SIZE,
        ),
        EntityInfo::ID => (EntityInfoArray::ID, *current_entity * EntityInfo::SIZE),
        EntityMemoryArray::ID => (EntityMemoryArray::ID, *current_entity * EntityMemory::SIZE),
        _ => (block_id, index),
    }
}
