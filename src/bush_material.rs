//! A shader that reads a mesh's custom vertex attribute.

use bevy::{
    asset::weak_handle,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexFormat},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

const PATH: &str = "embedded://bush.wgsl";

pub const QUAD_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("QUAD_INDEX", 09809767611, VertexFormat::Uint32);

// const RIPPLES_IMAGE_HANDLE: Handle<Image> = weak_handle!("42b28e6d-f814-494e-9b91-c58e4ea3a050");

pub(crate) struct BushMaterialPlugin;
impl Plugin for BushMaterialPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(build_host_windows)]
        embedded_asset!(app, "src\\", "bush.wgsl");
        #[cfg(not(build_host_windows))]
        embedded_asset!(app, "src/", "bush.wgsl");

        app.add_plugins(MaterialPlugin::<BushMaterial>::default());
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, Debug, Clone, TypePath, Default)]
pub struct BushMaterial {
    #[uniform(0)]
    pub light: LinearRgba,
    #[uniform(0)]
    pub mid: LinearRgba,
    #[uniform(0)]
    pub dark: LinearRgba,
    #[uniform(0)]
    pub offset_size: f32,

    #[uniform(0)]
    pub rotation_factor: f32,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material for BushMaterial {
    fn vertex_shader() -> ShaderRef {
        PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            QUAD_INDEX.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
