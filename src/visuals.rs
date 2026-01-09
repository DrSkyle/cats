use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<MetaballMaterial>::default());
        // app.add_systems(Startup, setup_metaballs);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MetaballMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub threshold: f32,
    // Add more uniforms for sphere positions/counts if doing raymarching in frag shader
}

impl Material for MetaballMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/metaball.wgsl".into()
    }
}
