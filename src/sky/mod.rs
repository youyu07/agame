use bevy::{core::Bytes, prelude::*, reflect::TypeUuid, render::{
        camera::Camera,
        pipeline::*,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::*,
        shader::{ShaderStage, ShaderStages},
        texture::TextureFormat,
    }};

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<SkyMaterial>()
            .add_startup_system(setup.system())
            .add_system(camera_update.system())
            .add_system(light_update.system());
    }
}

#[derive(Bytes, RenderResources, RenderResource, TypeUuid)]
#[render_resources(from_self)]
#[uuid = "20b628fd-9f2b-4c28-b71c-9ee22d77f242"]
pub struct SkyMaterial {
    pub camera_view: Mat4,
    pub sun_position: Vec3,
    pub sun_intensity_factor: f32,
    pub mie_k_coefficient: Vec3,
    pub sun_intensity_falloff_steepness: f32,
    pub primaries: Vec3,
    pub sun_angular_diameter_degrees: f32,
    pub luminance: f32,
    pub mie_coefficient: f32,
    pub mie_directional_g: f32,
    pub mie_v: f32,
    pub mie_zenith_length: f32,
    pub depolarization_factor: f32,
    pub num_molecules: f32,
    pub rayleigh: f32,
    pub rayleigh_zenith_length: f32,
    pub refractive_index: f32,
    pub tonemap_weighting: f32,
    pub turbidity: f32,
}

impl Default for SkyMaterial {
    fn default() -> Self {
        Self {
            camera_view: Mat4::identity(),
            sun_position: Vec3::new(0.0, 70000.0, 0.0),
            sun_intensity_factor: 1000.0,
            mie_k_coefficient: Vec3::new(0.686, 0.678, 0.666),
            sun_intensity_falloff_steepness: 1.5,
            primaries: Vec3::new(6.8e-7, 5.5e-7, 4.5e-7),
            sun_angular_diameter_degrees: 0.0093333,
            luminance: 1.0,
            mie_coefficient: 0.005,
            mie_directional_g: 0.8,
            mie_v: 4.0,
            mie_zenith_length: 1.25e3,
            depolarization_factor: 0.035,
            num_molecules: 2.542e25,
            rayleigh: 1.0,
            rayleigh_zenith_length: 8.4e3,
            refractive_index: 1.0003,
            tonemap_weighting: 9.5,
            turbidity: 2.0,
        }
    }
}

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline = {
        let vert_shader = Shader::from_glsl(ShaderStage::Vertex, include_str!("sky.vert"));
        let frag_shader = Shader::from_glsl(ShaderStage::Fragment, include_str!("sky.frag"));
        let mut desc = PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(vert_shader),
            fragment: Some(shaders.add(frag_shader)),
        });
        desc.depth_stencil_state = Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Always,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        });
        desc.rasterization_state = Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Front,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        });
        let handle = pipelines.add(desc);
        RenderPipeline::new(handle)
    };

    render_graph.add_system_node("sky_material", AssetRenderResourcesNode::<SkyMaterial>::new(true));
    render_graph
        .add_node_edge("sky_material", base::node::MAIN_PASS)
        .unwrap();

    let material = materials.add(SkyMaterial::default());

    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere{radius:2000.0,subdivisions: 5})),
            //mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![pipeline]),
            ..Default::default()
        })
        .with(material);
}

fn camera_update(
    mut sky_assets: ResMut<Assets<SkyMaterial>>,
    query_camera: Query<&GlobalTransform, With<Camera>>,
    query_material: Query<&Handle<SkyMaterial>>,
) {
    let mut matrix = Mat4::identity();
    for camera in query_camera.iter() {
        matrix = camera.compute_matrix();
    }

    for handle in query_material.iter() {
        if let Some(sky) = sky_assets.get_mut(handle) {
            sky.camera_view = matrix;
        }
    }
}


fn light_update(
    mut sky_assets: ResMut<Assets<SkyMaterial>>,
    query_light: Query<&GlobalTransform, With<Light>>,
    query_material: Query<&Handle<SkyMaterial>>,
) {
    let mut position = Vec3::zero();
    for light in query_light.iter() {
        position = light.translation;
    }

    for handle in query_material.iter() {
        if let Some(sky) = sky_assets.get_mut(handle) {
            sky.sun_position = position;
        }
    }
}