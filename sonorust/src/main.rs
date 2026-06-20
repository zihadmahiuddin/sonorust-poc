#![allow(unused)]
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    io::{BufReader, Cursor, Read},
    sync::Arc,
};

use bevy::{
    asset::RenderAssetUsages,
    ecs::{entity::Entity as BevyEntity, system::SystemParam},
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::{
        camera::ScalingMode,
        mesh::{Indices, PrimitiveTopology},
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::PrimaryWindow,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use image::{GenericImageView, ImageReader};
use ordered_float::OrderedFloat;
use rand::Rng;
use sonorust_interpreter::{
    Executor, IterativeInterpreter,
    node::ResolvedNode,
    opcode::{SideEffectAccess, TimingAccess, print_node_tree},
    side_effect::{DrawSideEffect, SideEffect, SideEffectKind, SpawnSideEffect},
};
use sonorust_memory::access::{
    InitializeMemoryAccess, PreprocessMemoryAccess, ShouldSpawnMemoryAccess,
    SpawnOrderMemoryAccess, TerminateMemoryAccess, UpdateParallelMemoryAccess,
    UpdateSequentialMemoryAccess,
};
use sonorust_model::{
    archetype::{
        ArchetypeId, data::EnginePlayDataArchetype, life::ArchetypeLife, score::ArchetypeScore,
    },
    engine::{
        configuration::EngineConfiguration,
        play_data::{Bucket, EnginePlayData},
        rom::EngineRom,
    },
    entity::{
        EntityId,
        data_array::{EntityData, EntityDataArray},
        despawn_array::EntityDespawn,
        info::{EntityInfoArray, EntityState},
        input::EntityInputArray,
        life::EntityLife,
        memory::EntityMemoryArray,
        score::EntityScore,
        shared_memory_array::EntitySharedMemoryArray,
    },
    level::{
        bucket::LevelBucket,
        data::{LevelData, LevelDataEntity, LevelDataEntityDataPayload, LevelDataMemory},
        life::LevelLife,
        memory::LevelMemory,
        option::LevelOption,
        score::LevelScore,
    },
    runtime::{
        background::RuntimeBackground, environment::RuntimeEnvironment,
        particle_transform::RuntimeParticleTransform, skin_transform::RuntimeSkinTransform,
        touch::RuntimeTouchArray, ui::RuntimeUi, ui_configuration::RuntimeUiConfiguration,
        update::RuntimeUpdate,
    },
    skin::data::{SkinData, SkinSpriteName, SkinSpriteTransform, SkinSpriteTransformExpression},
    temporary::TemporaryMemory,
};
use sonorust_rest::{client::SonorustRestClient, extension::LevelInfoExt};

fn main() {
    let server = "http://localhost:8080";
    let level = "dev";

    // let server = "https://sonolus.sekai.best";
    // let level = "sekai-best-429-1416-expert";

    // let server = "https://coconut.sonolus.com/horizon/";
    // let level = "coconut-horizon-83";

    let sonorust_plugin = SonorustPlugin::new(server, level);

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sonorust".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            AudioPlugin,
        ))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(sonorust_plugin)
        .run();
}

#[derive(Resource)]
struct LevelBgmSource(Handle<AudioSource>);

#[derive(Resource)]
struct LevelBgmInstance(Handle<AudioInstance>);

#[derive(Deref, DerefMut, Resource)]
struct EntityMap(BTreeMap<EntityId, Entity>);

#[derive(Default, Deref, DerefMut, Resource)]
struct SpawnQueue(BTreeMap<SpawnOrder, BTreeSet<EntityId>>);

#[derive(Default, Deref, DerefMut, Resource)]
struct InitializeQueue(BTreeSet<EntityId>);

#[derive(Default, Resource)]
struct SideEffects {
    pub spawns: BTreeMap<EntityId, Vec<SpawnSideEffect>>,
    pub draws: BTreeMap<EntityId, Vec<DrawSideEffect>>,
}

impl SideEffectAccess for SideEffects {
    fn add(&mut self, side_effect: SideEffect) {
        match side_effect.kind {
            SideEffectKind::Spawn(spawn_side_effect) => {
                self.spawns
                    .entry(side_effect.entity)
                    .and_modify(|effects| effects.push(spawn_side_effect.clone()))
                    .or_insert(vec![spawn_side_effect]);
            }
            SideEffectKind::Draw(draw_side_effect) => {
                self.draws
                    .entry(side_effect.entity)
                    .and_modify(|effects| effects.push(draw_side_effect.clone()))
                    .or_insert(vec![draw_side_effect]);
            }
        }
    }
}

#[derive(Deref, Event)]
struct ShouldSpawnEvent(EntityId);

fn preparation(
    mut commands: Commands,
    level_bgm_source: Res<LevelBgmSource>,
    audio: Res<Audio>,
    bgm_offset: Res<BgmOffset>,
    mut runtime_environment: ResMut<RuntimeEnvironment>,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = query_window.single().unwrap();

    runtime_environment.screen_aspect_ratio = (window.width() / window.height()) as f64;

    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 2.0,
        },
        viewport_origin: Vec2::new(0.5, 0.5),
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((Camera2d, projection));

    let instance_handle = audio
        .play(level_bgm_source.0.clone())
        .start_from(**bgm_offset)
        .with_volume(0.3)
        .handle();
    commands.insert_resource(LevelBgmInstance(instance_handle));
}

#[derive(SystemParam)]
struct TimingInfo<'w> {
    bpm_changes: Res<'w, BpmChanges>,
    time_scale_changes: Res<'w, TimeScaleChanges>,
}

impl<'w> TimingAccess for TimingInfo<'w> {
    fn beat_to_time(&self, beat: f64) -> f64 {
        self.bpm_changes.beat_to_time(beat)
    }

    fn beat_to_bpm(&self, beat: f64) -> f64 {
        self.bpm_changes.beat_to_bpm(beat)
    }

    fn time_to_scaled_time(&self, time: f64) -> f64 {
        self.time_scale_changes.time_to_scaled_time(time)
    }
}

fn preprocessing(
    timing: TimingInfo,
    entities: Res<EntityMap>,
    mut memory: PreprocessMemoryAccess,
    mut side_effects: ResMut<SideEffects>,
    nodes: Res<InterpreterNodes>,
) {
    // println!("Entering preprocessing");
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.0.iter() {
        let Some(preprocess_callback) = &entity.archetype.preprocess.as_ref() else {
            continue;
        };
        let order = preprocess_callback.order.unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeMap<_, _>| {
                entities.insert(entity_id, entity);
            })
            .or_insert(BTreeMap::from([(entity_id, entity)]));
    }

    for (_order, entity_ids) in callback_order_map {
        for (entity_id, &entity) in entity_ids.iter() {
            let Some(preprocess) = &entity.archetype.preprocess else {
                continue;
            };

            let mut interpreter = IterativeInterpreter::new(
                **entity_id,
                nodes.0.as_slice(),
                &mut memory,
                &mut *side_effects,
                &timing,
            );
            interpreter.execute(preprocess.index);
        }
    }
}

fn spawn_ordering(
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    entities: Res<EntityMap>,
    mut side_effects: ResMut<SideEffects>,
    mut spawn_queue: ResMut<SpawnQueue>,
    mut memory: SpawnOrderMemoryAccess,
) {
    // println!("Entering spawn ordering");
    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.0.iter() {
        let spawn_order_callback = &entity.archetype.spawn_order.as_ref();
        let order = spawn_order_callback
            .and_then(|callback| callback.order)
            .unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeMap<_, _>| {
                entities.insert(entity_id, entity);
            })
            .or_insert(BTreeMap::from([(entity_id, entity)]));
    }

    for (_, entity_ids) in callback_order_map {
        for (entity_id, &entity) in entity_ids.iter() {
            let order = match &entity.archetype.spawn_order {
                Some(spawn_order) => {
                    let mut interpreter = IterativeInterpreter::new(
                        **entity_id,
                        nodes.0.as_slice(),
                        &mut memory,
                        &mut *side_effects,
                        &timing,
                    );
                    interpreter.execute(spawn_order.index)
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
    level_bgm: Res<LevelBgmInstance>,
    bgm_offset: Res<BgmOffset>,
    audio_instances: Res<Assets<AudioInstance>>,
    mut runtime_update: ResMut<RuntimeUpdate>,
) {
    // println!("Setting runtime update values");
    let Some(level_bgm_audio) = audio_instances.get(&level_bgm.0) else {
        return;
    };

    let delta_time = time.delta_secs_f64();
    // let time = time.elapsed_secs_f64();
    let time = level_bgm_audio.state().position().unwrap_or_default() - bgm_offset.0;
    let scaled_time = time_scale_changes.time_to_scaled_time(time);

    let touch_count = 0.0; // TODO
    runtime_update.time = time;
    runtime_update.delta_time = delta_time;
    runtime_update.scaled_time = scaled_time;
    runtime_update.touch_count = touch_count;
}

fn should_spawn_callback(
    entities: Res<EntityMap>,
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    mut spawn_queue: ResMut<SpawnQueue>,
    mut side_effects: ResMut<SideEffects>,
    mut memory: ShouldSpawnMemoryAccess,
    mut should_spawn_events: EventWriter<ShouldSpawnEvent>,
) {
    // println!("Entering should spawn callback");
    let mut orders_to_remove = Vec::new();

    let mut callback_order_map = BTreeMap::new();
    for (entity_id, entity) in entities.0.iter() {
        let should_spawn_callback = &entity.archetype.should_spawn.as_ref();
        let order = should_spawn_callback
            .and_then(|callback| callback.order)
            .unwrap_or_default();
        callback_order_map
            .entry(order)
            .and_modify(|entities: &mut BTreeMap<_, _>| {
                entities.insert(entity_id, entity);
            })
            .or_insert(BTreeMap::from([(entity_id, entity)]));
    }

    let mut should_spawn_map = BTreeSet::new();
    for (_order, order_entities) in callback_order_map {
        for (entity_id, entity) in order_entities {
            let should_spawn = if let Some(should_spawn) = &entity.archetype.should_spawn {
                let mut interpreter = IterativeInterpreter::new(
                    *entity_id,
                    nodes.0.as_slice(),
                    &mut memory,
                    &mut *side_effects,
                    &timing,
                );
                interpreter.execute(should_spawn.index) != 0.0
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
    entities: Res<EntityMap>,
    mut commands: Commands,
    mut initialize_queue: ResMut<InitializeQueue>,
    mut should_spawn_events: EventReader<ShouldSpawnEvent>,
    mut entity_info_array: ResMut<EntityInfoArray>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_atlas_resources: Res<TextureAtlasResources>,
) {
    // println!("Entering spawning");
    initialize_queue.clear();

    for ShouldSpawnEvent(entity_id) in should_spawn_events.read() {
        initialize_queue.insert(*entity_id);
        if let Some(entity_info) = entity_info_array.entry_mut(entity_id) {
            entity_info.state = EntityState::Active;
        };

        let entity = &entities.0[entity_id];

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        // Initialize with a dummy quad that will be updated
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ],
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
        );
        mesh.insert_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2])); // Bevy quad order (BL, TL, TR, BR)
        let mesh_handle = meshes.add(mesh);

        let mesh = Mesh2d(mesh_handle);
        let material = MeshMaterial2d(texture_atlas_resources.atlas_material_handle.clone());

        let entity = &entities.0[entity_id];
        commands.spawn((
            mesh,
            material,
            Transform::IDENTITY,
            *entity_id,
            entity.clone(),
            Name::new(format!(
                "Entity {} ({})",
                entity.id.0, entity.archetype.name
            )),
        ));
    }
}

fn initialization(
    initialize_queue: Res<InitializeQueue>,
    entities: Res<EntityMap>,
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    mut side_effects: ResMut<SideEffects>,
    mut memory: InitializeMemoryAccess,
) {
    // println!("Entering initialization");
    let mut callback_order_map = BTreeMap::new();

    for entity_id in initialize_queue.iter() {
        let entity = &entities.0[entity_id];
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
            let entity = &entities.0[entity_id];
            let Some(initialize) = &entity.archetype.initialize else {
                continue;
            };
            let initialize_index = initialize.index;

            let mut interpreter = IterativeInterpreter::new(
                *entity_id,
                nodes.0.as_slice(),
                &mut memory,
                &mut *side_effects,
                &timing,
            );
            interpreter.execute(initialize_index);
        }
    }
}

fn sequential_update(
    entities: Query<&Entity>,
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    mut side_effects: ResMut<SideEffects>,
    mut memory: UpdateSequentialMemoryAccess,
) {
    // println!("Entering sequential update");
    let mut callback_order_map = BTreeMap::new();
    let mut entity_id_to_entity_map = HashMap::new();

    for entity in entities {
        entity_id_to_entity_map.insert(entity.id, entity);
    }

    for (entity_id, entity) in entity_id_to_entity_map.iter() {
        if memory.entity_info_array.entry(entity_id).unwrap().state != EntityState::Active {
            continue;
        }

        if let Some(update_sequential_callback) = &entity.archetype.update_sequential {
            let order = update_sequential_callback.order.unwrap_or_default();
            callback_order_map
                .entry(order)
                .and_modify(|entities: &mut BTreeSet<_>| {
                    entities.insert(entity_id);
                })
                .or_insert(BTreeSet::from([entity_id]));
        }
    }

    for (_order, order_entities) in callback_order_map {
        for entity_id in order_entities.iter() {
            let entity = entity_id_to_entity_map[entity_id];
            let Some(update_sequential) = &entity.archetype.update_sequential else {
                continue;
            };
            let update_sequential_index = update_sequential.index;

            let mut interpreter = IterativeInterpreter::new(
                **entity_id,
                nodes.0.as_slice(),
                &mut memory,
                &mut *side_effects,
                &timing,
            );
            interpreter.execute(update_sequential_index);
        }
    }
}

fn input() {}

fn parallel_update(
    entities: Query<&Entity>,
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    mut side_effects: ResMut<SideEffects>,
    mut memory: UpdateParallelMemoryAccess,
) {
    // println!("Entering parallel update");
    let mut callback_order_map = BTreeMap::new();
    let mut entity_id_to_entity_map = HashMap::new();

    for entity in entities {
        entity_id_to_entity_map.insert(entity.id, entity);
    }

    for (entity_id, entity) in entity_id_to_entity_map.iter() {
        let Some(entity_info) = memory.entity_info_array.entry(entity_id) else {
            continue;
        };

        if entity_info.state != EntityState::Active {
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
            let entity = &entity_id_to_entity_map[entity_id];
            let Some(update_parallel) = &entity.archetype.update_parallel else {
                continue;
            };
            let update_parallel_index = update_parallel.index;

            let mut interpreter = IterativeInterpreter::new(
                *entity_id,
                nodes.0.as_slice(),
                &mut memory,
                &mut *side_effects,
                &timing,
            );
            interpreter.execute(update_parallel_index);
        }
    }
}

fn despawning(
    mut commands: Commands,
    entities: Query<(bevy::ecs::entity::Entity, &EntityId)>,
    mut entity_despawn: ResMut<EntityDespawn>,
    mut entity_info_array: ResMut<EntityInfoArray>,
) {
    // println!("Despawning");

    let entity_map = entities
        .into_iter()
        .map(|(a, b)| (*b, a))
        .collect::<HashMap<_, _>>();

    let despawned_entities = entity_despawn.items.clone();
    for despawned_entity_id in despawned_entities {
        let entity_info = entity_info_array.entry_mut(&despawned_entity_id).unwrap();
        if entity_info.state == EntityState::Despawned {
            continue;
        }

        entity_info.state = EntityState::Despawned;
        entity_despawn.remove(&despawned_entity_id);
        if let Some(entity) = entity_map.get(&despawned_entity_id) {
            commands.entity(*entity).despawn();
        }
    }
}

fn terminate_callback(
    entities: Query<&Entity>,
    timing: TimingInfo,
    nodes: Res<InterpreterNodes>,
    mut side_effects: ResMut<SideEffects>,
    mut memory: TerminateMemoryAccess,
) {
    // println!("Entering terminate");
    let mut callback_order_map = BTreeMap::new();
    let mut entity_id_to_entity_map = HashMap::new();

    for entity in entities {
        entity_id_to_entity_map.insert(entity.id, entity);
    }

    for despawned_entity_id in memory.entity_despawn.iter() {
        let entity = &entity_id_to_entity_map[despawned_entity_id];
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
            let entity = &entity_id_to_entity_map[entity_id];
            let Some(terminate) = &entity.archetype.terminate else {
                continue;
            };
            let terminate_index = terminate.index;

            let mut interpreter = IterativeInterpreter::new(
                *entity_id,
                nodes.0.as_slice(),
                &mut memory,
                &mut *side_effects,
                &timing,
            );
            interpreter.execute(terminate_index);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn presentation(
    entities: Query<((&mut Transform, &Mesh2d, &EntityId))>,
    mut meshes: ResMut<Assets<Mesh>>,
    sonolus_sprite_lookup: Res<SonolusSpriteLookup>,
    atlas_resources: Res<TextureAtlasResources>, // For atlas image info
    images: Res<Assets<Image>>,
    side_effects: Res<SideEffects>,
    skin_transform: Res<RuntimeSkinTransform>,
) {
    let Some(atlas_image) = images.get(&atlas_resources.atlas_image_handle) else {
        error!("Atlas image not found for UV calculation.");
        return;
    };
    let atlas_size = Vec2::new(atlas_image.width() as f32, atlas_image.height() as f32);

    let mut entity_map = entities
        .into_iter()
        .map(|(a, b, c)| (*c, (a, b)))
        .collect::<HashMap<_, _>>();

    for (entity_id, effects) in &side_effects.draws {
        let Some((_entity_transform, mesh_2d)) = entity_map.get_mut(entity_id) else {
            error!(
                "Entity {} could not be drawn as it was not found in the entity map.",
                entity_id.0
            );
            continue;
        };

        let Some(mesh) = meshes.get_mut(&mesh_2d.0) else {
            error!("Mesh not found for SonolusSpriteRender.");
            continue;
        };

        let mut quad_corners = Vec::new();
        let mut quad_uvs = Vec::new();
        let mut quad_indices = Vec::new();
        let mut vertex_offset: u32 = 0;

        for effect in effects {
            let Some((_sprite_name, atlas_rect, transform)) =
                sonolus_sprite_lookup.id_to_data.get(&effect.sprite_id)
            else {
                error!(
                    "Sonolus sprite data not found for atlas id: {}",
                    effect.sprite_id
                );
                continue;
            };

            let bl = Vec3::new(effect.x1 as f32, effect.y1 as f32, effect.z as f32);
            let tl = Vec3::new(effect.x2 as f32, effect.y2 as f32, effect.z as f32);
            let tr = Vec3::new(effect.x3 as f32, effect.y3 as f32, effect.z as f32);
            let br = Vec3::new(effect.x4 as f32, effect.y4 as f32, effect.z as f32);

            let bl = skin_transform.0.transform_point3(bl);
            let tl = skin_transform.0.transform_point3(tl);
            let tr = skin_transform.0.transform_point3(tr);
            let br = skin_transform.0.transform_point3(br);

            let input_x1 = bl.x;
            let input_y1 = bl.y;
            let input_x2 = tl.x;
            let input_y2 = tl.y;
            let input_x3 = tr.x;
            let input_y3 = tr.y;
            let input_x4 = br.x;
            let input_y4 = br.y;

            // Apply the Sonolus transform expressions
            let out_x1 = apply_sonolus_transform_expression(
                &transform.x1,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_y1 = apply_sonolus_transform_expression(
                &transform.y1,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_x2 = apply_sonolus_transform_expression(
                &transform.x2,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_y2 = apply_sonolus_transform_expression(
                &transform.y2,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_x3 = apply_sonolus_transform_expression(
                &transform.x3,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_y3 = apply_sonolus_transform_expression(
                &transform.y3,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_x4 = apply_sonolus_transform_expression(
                &transform.x4,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );
            let out_y4 = apply_sonolus_transform_expression(
                &transform.y4,
                (input_x1, input_y1),
                (input_x2, input_y2),
                (input_x3, input_y3),
                (input_x4, input_y4),
            );

            // Output points matching Bevy's quad vertex order (BL, TL, TR, BR)
            let output_corners: [Vec3; 4] = [
                Vec3::new(out_x1, out_y1, effect.z as f32), // Bottom-left
                Vec3::new(out_x2, out_y2, effect.z as f32), // Top-left
                Vec3::new(out_x3, out_y3, effect.z as f32), // Top-right
                Vec3::new(out_x4, out_y4, effect.z as f32), // Bottom-right
            ];
            quad_corners.extend_from_slice(&output_corners);

            // --- Update UVs ---
            let min_x_uv = atlas_rect.min.x as f32 / atlas_size.x;
            let max_x_uv = atlas_rect.max.x as f32 / atlas_size.x;
            let min_y_uv = atlas_rect.min.y as f32 / atlas_size.y; // Top of the sprite in atlas
            let max_y_uv = atlas_rect.max.y as f32 / atlas_size.y; // Bottom of the sprite in atlas

            // These UVs must match the vertex order of the mesh: BL, TL, TR, BR
            let uvs = vec![
                [min_x_uv, max_y_uv], // Bottom-left UV
                [min_x_uv, min_y_uv], // Top-left UV
                [max_x_uv, min_y_uv], // Top-right UV
                [max_x_uv, max_y_uv], // Bottom-right UV
            ];
            quad_uvs.extend_from_slice(&uvs);
            quad_indices.extend_from_slice(&[
                vertex_offset,
                vertex_offset + 1,
                vertex_offset + 2,
                vertex_offset,
                vertex_offset + 2,
                vertex_offset + 3,
            ]);
            vertex_offset += 4;
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, quad_corners);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, quad_uvs);
        mesh.insert_indices(Indices::U32(quad_indices));
    }
}

#[derive(Debug)]
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

#[derive(Component, Clone)]
pub struct Entity {
    id: EntityId,
    archetype_id: ArchetypeId,
    archetype: Arc<EnginePlayDataArchetype>,
    data: EntityData,
}

enum SpecialArchetype {
    BpmChange { beat: f64, bpm: f64 },
    TimescaleChange { beat: f64, timescale: f64 },
}

impl TryFrom<LevelDataEntity> for SpecialArchetype {
    type Error = ();

    fn try_from(value: LevelDataEntity) -> Result<Self, Self::Error> {
        let mut data = value
            .data
            .into_iter()
            .map(|data| (data.name, data.payload))
            .collect::<HashMap<_, _>>();
        let beat = match data.remove("#BEAT") {
            Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
            _ => return Err(()),
        };
        match value.archetype.as_str() {
            "#BPM_CHANGE" => {
                let bpm = match data.remove("#BPM") {
                    Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
                    _ => return Err(()),
                };
                Ok(SpecialArchetype::BpmChange { beat, bpm })
            }
            "#TIMESCALE_CHANGE" => {
                let timescale = match data.remove("#TIMESCALE") {
                    Some(Some(LevelDataEntityDataPayload::Value { value })) => value,
                    _ => return Err(()),
                };
                Ok(SpecialArchetype::TimescaleChange { beat, timescale })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Resource)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
struct SpawnOrder(OrderedFloat<f64>);

impl From<f64> for SpawnOrder {
    fn from(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
}

#[derive(Debug, Resource)]
struct SonolusServer(String);
#[derive(Debug, Resource)]
struct SonolusLevel(String);

struct SonorustPlugin {
    // engine_play_data: EnginePlayData,
    // engine_configuration: EngineConfiguration,
    // level_data: LevelData,
    // skin_data: SkinData,
    // skin_texture_bytes: SkinTextureBytes,
    // level_bgm_bytes: LevelBgmBytes,
    server: String,
    level: String,
}

impl SonorustPlugin {
    pub fn new(server: &str, level: &str) -> Self {
        Self {
            server: server.to_string(),
            level: level.to_string(),
        }
    }

    pub fn add_memories(
        commands: &mut Commands,
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
        let level_data = LevelDataMemory::default();
        let level_option = LevelOption::new(&engine_configuration.options);
        let level_bucket = LevelBucket::new(buckets.len());
        let level_score = LevelScore::default();
        let level_life = LevelLife::default();

        let engine_rom = EngineRom::default();

        let entity_memory_array = EntityMemoryArray::new(entities.keys().copied());
        // TODO: maybe construct the data and give it to the block directly instead of cloning
        let entity_data_array = EntityDataArray::new(
            entities
                .values()
                .map(|entity| (entity.id, entity.data.clone())),
        );
        let entity_shared_memory_array = EntitySharedMemoryArray::new(entities.len());
        let entity_info_array = EntityInfoArray::new(
            entities
                .iter()
                .map(|(entity_id, entity)| (entity_id, entity.archetype_id)),
        );
        let entity_despawn = EntityDespawn::default();
        let entity_input_array = EntityInputArray::new(entities.keys());
        let entity_life = EntityLife::new(entities.len());
        let entity_score = EntityScore::new(entities.len());

        let archetype_life = ArchetypeLife::new(archetypes.len());
        let archetype_score = ArchetypeScore::new(archetypes.len());

        let temporary_memory = TemporaryMemory::default();

        commands.insert_resource(runtime_environment);
        commands.insert_resource(runtime_update);
        commands.insert_resource(runtime_touch_array);
        commands.insert_resource(runtime_skin_transform);
        commands.insert_resource(runtime_particle_transform);
        commands.insert_resource(runtime_background);
        commands.insert_resource(runtime_ui);
        commands.insert_resource(runtime_ui_configuration);
        commands.insert_resource(level_memory);
        commands.insert_resource(level_data);
        commands.insert_resource(level_option);
        commands.insert_resource(level_bucket);
        commands.insert_resource(level_score);
        commands.insert_resource(level_life);
        commands.insert_resource(engine_rom);
        commands.insert_resource(entity_memory_array);
        commands.insert_resource(entity_data_array);
        commands.insert_resource(entity_shared_memory_array);
        commands.insert_resource(entity_info_array);
        commands.insert_resource(entity_despawn);
        commands.insert_resource(entity_input_array);
        commands.insert_resource(entity_life);
        commands.insert_resource(entity_score);
        commands.insert_resource(archetype_life);
        commands.insert_resource(archetype_score);
        commands.insert_resource(temporary_memory);
    }
}

impl Plugin for SonorustPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SonolusServer(self.server.clone()))
            .insert_resource(SonolusLevel(self.level.clone()))
            .add_event::<ShouldSpawnEvent>()
            .add_systems(PreStartup, pre_startup)
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
            .add_systems(Last, clear_side_effects);
    }
}

#[allow(clippy::too_many_arguments)]
fn pre_startup(
    mut commands: Commands,
    server: Res<SonolusServer>,
    level: Res<SonolusLevel>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut audio_sources: ResMut<Assets<AudioSource>>,
) {
    let sonorust_client = SonorustRestClient::new(&server.0).unwrap();
    let level_info = sonorust_client.level_info(&level.0).unwrap();
    let engine_play_data = level_info.engine_play_data(&sonorust_client).unwrap();
    let engine_configuration = level_info.engine_configuration(&sonorust_client).unwrap();
    let level_data = level_info.data(&sonorust_client).unwrap();
    let skin_data = level_info.skin_data(&sonorust_client).unwrap();
    let skin_texture_bytes = level_info.skin_texture_bytes(&sonorust_client).unwrap();
    let level_bgm_bytes = level_info.bgm_bytes(&sonorust_client).unwrap();
    let archetypes = engine_play_data
        .archetypes
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, archetype)| (archetype.name.clone(), (index, Arc::new(archetype.clone()))))
        .collect::<HashMap<_, _>>();

    let bgm_offset = BgmOffset(level_data.bgm_offset);

    let (entities, mut bpm_changes, mut time_scale_changes) =
        level_data.entities.clone().into_iter().enumerate().fold(
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
                                    warn!(
                                        "TODO: Level Data Entity Data Payload by reference {reference}"
                                    );
                                }
                                LevelDataEntityDataPayload::Value { value } => {
                                    entity_data[import.index] = *value;
                                }
                            }
                        }
                    }

                    let entity_id = EntityId(entity_index);
                    entities.insert(
                        entity_id,
                        Entity {
                            // name: entity.name,
                            id: entity_id,
                            archetype_id: ArchetypeId(*archetype_index),
                            archetype: Arc::clone(archetype),
                            data: EntityData::new(entity_data),
                        },
                    );
                } else if let Ok(special_archetype) = SpecialArchetype::try_from(entity) {
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
    let bpm_changes = bpm_changes
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

    time_scale_changes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

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
    let (oks, errs) = engine_play_data
        .nodes
        .clone()
        .into_iter()
        .map(ResolvedNode::try_from)
        .partition::<Vec<_>, _>(|r| r.is_ok());

    let ok_nodes: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
    let err_nodes: HashSet<_> = errs.into_iter().map(Result::unwrap_err).collect();

    if !err_nodes.is_empty() {
        panic!("Failed to resolve nodes: {err_nodes:#?}");
    }

    let interpreter_nodes = InterpreterNodes(ok_nodes);

    SonorustPlugin::add_memories(
        &mut commands,
        &archetypes,
        &entities,
        &engine_play_data.buckets,
        &engine_configuration,
    );

    // TODO: spawn entities
    commands.insert_resource(EntityMap(entities));
    commands.insert_resource(interpreter_block_stack);
    commands.insert_resource(interpreter_nodes);
    commands.insert_resource(bpm_changes);
    commands.insert_resource(time_scale_changes);
    commands.insert_resource(bgm_offset);
    commands.insert_resource(SpawnQueue::default());
    commands.insert_resource(InitializeQueue::default());
    commands.insert_resource(SideEffects::default());

    let audio_source = audio_sources.add(AudioSource {
        sound: StaticSoundData::from_cursor(Cursor::new(level_bgm_bytes.0.clone())).unwrap(),
    });
    commands.insert_resource(LevelBgmSource(audio_source));

    let decoded_image = ImageReader::new(Cursor::new(&skin_texture_bytes.0))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let (atlas_width, atlas_height) = decoded_image.dimensions();
    let rgba8_data = decoded_image.as_rgba8().unwrap();

    let image_sampler = if skin_data.interpolation {
        ImageSampler::linear()
    } else {
        ImageSampler::nearest()
    };

    let mut image = Image::new(
        Extent3d {
            width: atlas_width,
            height: atlas_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba8_data.to_vec(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.sampler = image_sampler;
    let atlas_image_handle = images.add(image);
    // commands.insert_resource(SkinTextureHandle(atlas_image_handle.clone()));

    let mut texture_rects: Vec<URect> = Vec::new();
    let mut id_to_data: HashMap<usize, (SkinSpriteName, URect, SkinSpriteTransform)> =
        HashMap::new();
    let sprite_name_to_data = skin_data
        .sprites
        .clone()
        .into_iter()
        .map(|sprite_data| (sprite_data.name.0.clone(), sprite_data))
        .collect::<HashMap<_, _>>();

    // The order of sprites in SonolusSkinData.sprites must match the order
    // you expect for atlas_index (e.g., first sprite in JSON is atlas_index 0, etc.)
    for engine_skin_sprite in engine_play_data.skin.sprites.iter() {
        let Some(sprite_data) = sprite_name_to_data.get(&engine_skin_sprite.name) else {
            continue;
        };

        let rect = URect::new(
            sprite_data.x as u32,
            sprite_data.y as u32,
            (sprite_data.x + sprite_data.w) as u32,
            (sprite_data.y + sprite_data.h) as u32,
        );
        texture_rects.push(rect);
        id_to_data.insert(
            engine_skin_sprite.id,
            (
                sprite_data.name.clone(),
                rect,
                sprite_data.transform.clone(),
            ),
        );
    }

    let texture_atlas_layout = TextureAtlasLayout {
        size: UVec2::new(atlas_width, atlas_height),
        textures: texture_rects,
    };
    let atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);

    commands.insert_resource(TextureAtlasResources {
        atlas_image_handle: atlas_image_handle.clone(),
        atlas_material_handle: materials.add(ColorMaterial::from(atlas_image_handle)),
    });
    commands.insert_resource(SonolusSpriteLookup {
        id_to_data,
        atlas_layout_handle: atlas_layout_handle.clone(),
    });
}

// Resource to hold the pre-loaded atlas image and material
#[derive(Resource)]
pub struct TextureAtlasResources {
    pub atlas_image_handle: Handle<Image>,
    pub atlas_material_handle: Handle<ColorMaterial>,
}

// Resource to map atlas_index to Sonolus sprite data for quick lookup
#[derive(Resource)]
pub struct SonolusSpriteLookup {
    // Maps atlas_index to the SonolusSkinDataSprite (containing Rect and Transform data)
    // We store (sprite_name, original_rect, transform)
    pub id_to_data: HashMap<usize, (SkinSpriteName, URect, SkinSpriteTransform)>,
    pub atlas_layout_handle: Handle<TextureAtlasLayout>, // Bevy's internal atlas layout
}

fn clear_side_effects(mut side_effects: ResMut<SideEffects>) {
    side_effects.spawns.clear();
    side_effects.draws.clear();
}

// --- Helper function for Sonolus transform calculation ---
fn apply_sonolus_transform_expression(
    expression: &SkinSpriteTransformExpression,
    (input_x1, input_y1): (f32, f32),
    (input_x2, input_y2): (f32, f32),
    (input_x3, input_y3): (f32, f32),
    (input_x4, input_y4): (f32, f32),
) -> f32 {
    let mut output = 0.0;
    if let Some(x1_multiplier) = expression.x1 {
        output += input_x1 * x1_multiplier;
    }
    if let Some(y1_multiplier) = expression.y1 {
        output += input_y1 * y1_multiplier;
    }
    if let Some(x2_multiplier) = expression.x2 {
        output += input_x2 * x2_multiplier;
    }
    if let Some(y2_multiplier) = expression.y2 {
        output += input_y2 * y2_multiplier;
    }
    if let Some(x3_multiplier) = expression.x3 {
        output += input_x3 * x3_multiplier;
    }
    if let Some(y3_multiplier) = expression.y3 {
        output += input_y3 * y3_multiplier;
    }
    if let Some(x4_multiplier) = expression.x4 {
        output += input_x4 * x4_multiplier;
    }
    if let Some(y4_multiplier) = expression.y4 {
        output += input_y4 * y4_multiplier;
    }

    output
}

#[derive(Deref, Resource)]
struct InterpreterNodes(Vec<ResolvedNode>);

#[derive(Deref, DerefMut, Default, Resource)]
struct InterpreterBlockStack(usize);

#[derive(Deref, DerefMut, Resource)]
struct BgmOffset(f64);
