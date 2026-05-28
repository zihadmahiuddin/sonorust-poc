use bevy::{
    asset::RenderAssetUsages,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_resource::{
            AsBindGroup, BufferInitDescriptor, BufferUsages, Extent3d, RenderPipelineDescriptor,
            ShaderRef, ShaderType, SpecializedMeshPipelineError, TextureDimension, TextureFormat,
        },
        renderer::{RenderDevice, RenderQueue},
    },
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, ShaderType)]
struct QuadCorners {
    top_left: Vec2,
    top_right: Vec2,
    bottom_left: Vec2,
    bottom_right: Vec2,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct NormalUvMaterial {
    #[uniform(0)]
    quad_corners: QuadCorners,
    #[texture(1)]
    #[sampler(2)]
    base_color_texture: Handle<Image>,
}

impl Material for NormalUvMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/normal_uv.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/normal_uv.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?];
        Ok(())
    }
}

fn main() {
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
        .add_plugins(MaterialPlugin::<NormalUvMaterial>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, update_quad_corners)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NormalUvMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // --- camera ---
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
    ));

    // --- light ---
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 4.0, 4.0)));

    // --- checkerboard texture ---
    let size = 64;
    let mut data = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        for x in 0..size {
            let c = if ((x ^ y) & 8) != 0 { 255 } else { 0 };
            data.extend_from_slice(&[c, c, c, 255]);
        }
    }

    let image = Image::new_fill(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    let texture = images.add(image);

    // --- distorted quad geometry ---
    //
    // A ----- B
    // |     / |
    // |   /   |
    // | /     |
    // C ----- D

    let a = Vec3::new(-1.2, -0.8, 0.0);
    let b = Vec3::new(1.0, -0.4, 0.0);
    let c = Vec3::new(-0.6, 1.0, 0.0);
    let d = Vec3::new(1.2, 0.8, 0.0);

    // let a = Vec3::new(-1.0, -1.0, 0.0);
    // let b = Vec3::new(1.0, -1.0, 0.0);
    // let c = Vec3::new(-1.0, 1.0, 0.0);
    // let d = Vec3::new(1.0, 1.0, 0.0);

    // let corners = QuadCorners {
    //     top_left: Vec2::new(0.0, 1.0),
    //     top_right: Vec2::new(1.0, 1.0),
    //     bottom_left: Vec2::new(0.0, 0.0),
    //     bottom_right: Vec2::new(1.0, 0.0),
    // };

    // Map your Vec3 points to roughly where they appear in 0.0 - 1.0 screen space
    let corners = QuadCorners {
        // a is bottom-left-ish
        bottom_left: Vec2::new(0.32, 0.38),
        // b is bottom-right-ish
        bottom_right: Vec2::new(0.65, 0.43),
        // c is top-left-ish
        top_left: Vec2::new(0.40, 0.62),
        // d is top-right-ish
        top_right: Vec2::new(0.68, 0.60),
    };

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    // positions
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            a.to_array(),
            b.to_array(),
            c.to_array(),
            b.to_array(),
            d.to_array(),
            c.to_array(),
        ],
    );

    // UVs (normal interpolation)
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ],
    );

    // simple normals (all facing camera)
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 6]);

    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 3, 4, 5]));

    let mesh = meshes.add(mesh);

    // --- material ---
    let material = materials.add(NormalUvMaterial {
        base_color_texture: texture,
        quad_corners: corners,
    });

    // --- spawn ---
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material)));
}

fn update_quad_corners(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut materials: ResMut<Assets<NormalUvMaterial>>,
    material_handle: Query<&MeshMaterial3d<NormalUvMaterial>>,
) {
    let (camera, camera_transform) = camera_q.single().unwrap();

    // Your 4 world-space points
    let points = [
        Vec3::new(-1.2, -0.8, 0.0), // a
        Vec3::new(1.0, -0.4, 0.0),  // b
        Vec3::new(-0.6, 1.0, 0.0),  // c
        Vec3::new(1.2, 0.8, 0.0),   // d
    ];

    let mut projected = [Vec2::ZERO; 4];
    for (i, p) in points.iter().enumerate() {
        // Project 3D world space to 2D screen space (0.0 to 1.0)
        if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, *p) {
            // Normalize based on window size to get 0.0 - 1.0
            // Bevy's world_to_viewport returns pixels; we need 0..1
            // Note: You might need the window size here or use ndc directly
            projected[i] = screen_pos / camera.logical_viewport_size().unwrap();

            // FLIP Y: Screen space is top-down, but your shader
            // likely expects bottom-up based on your frag_pos calc
            projected[i].y = 1.0 - projected[i].y;
        }
    }

    if let Ok(mat_handle) = material_handle.single() {
        if let Some(mat) = materials.get_mut(mat_handle) {
            dbg!(projected);
            mat.quad_corners.bottom_left = projected[0];
            mat.quad_corners.bottom_right = projected[1];
            mat.quad_corners.top_left = projected[2];
            mat.quad_corners.top_right = projected[3];
        }
    }
}
