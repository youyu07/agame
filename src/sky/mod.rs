use bevy::{
    prelude::*,
    render::{
        texture::TextureFormat,
        pipeline::*,
        shader::{ShaderStage, ShaderStages},
    },
};

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const VERTEX_SHADER: &str = include_str!("sky.vert");
    const FRAGMENT_SHADER: &str = include_str!("sky.frag");

    let pipeline = {
        let mut desc = PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
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
        desc.rasterization_state = Some(
            RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Front,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }
        );
        pipelines.add(desc)
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere{radius:20000.0,subdivisions: 5})),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
        ..Default::default()
    });
}
