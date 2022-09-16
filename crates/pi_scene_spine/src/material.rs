use std::hash;

use pi_scene_geometry::{TVertexDataKindKey, geometry::GeometryDataDesc, vertex_data::EVertexDataFormat};
use pi_scene_material::{material::{Material, TMaterialBlockKindKey, MaterialUniformDesc, MaterialAttributeDesc, EUniformDataFormat, UnifromData}, texture::TextureKey};
use pi_scene_math::{Matrix, Vector4};

use crate::shaders::SpineShaderPool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, hash::Hash)]
pub enum SpineVertexDataKindKey {
    Vertices,
}
impl TVertexDataKindKey for SpineVertexDataKindKey {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, hash::Hash)]
pub enum SpineMaterialBlockKindKey{
    Vertices,
    MVPMatrix,
    MaskFlag,
    Texture,
}
impl TMaterialBlockKindKey for SpineMaterialBlockKindKey {}

pub struct SpineMaterialColored {}
impl SpineMaterialColored {
    pub fn init<SP: SpineShaderPool, K2D: TextureKey>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        spine_shader_pool: &SP
    ) {
        mat.init(
            device,
            vec![
                MaterialAttributeDesc {
                    vertex: GeometryDataDesc (SpineVertexDataKindKey::Vertices, EVertexDataFormat::F32),
                    bind: SpineMaterialBlockKindKey::Vertices
                }
            ],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            vec![
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4 },
            ],
            vec![],
            &spine_shader_pool.get_spine_shader_colored().bind_group_layouts
        );
    }
}


pub struct SpineMaterialColoredTextured {}
impl SpineMaterialColoredTextured {
    pub fn init<SP: SpineShaderPool, K2D: TextureKey>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        spine_shader_pool: &SP
    ) {
        mat.init(
            device,
            vec![
                MaterialAttributeDesc {
                    vertex: GeometryDataDesc (SpineVertexDataKindKey::Vertices, EVertexDataFormat::F32),
                    bind: SpineMaterialBlockKindKey::Vertices
                }
            ],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            vec![
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4 },
            ],
            vec![
                SpineMaterialBlockKindKey::Texture
            ],
            &spine_shader_pool.get_spine_shader_colored().bind_group_layouts
        );
    }
}

pub struct SpineMaterialColoredTexturedTwo {}
impl SpineMaterialColoredTexturedTwo {
    pub fn init<SP: SpineShaderPool, K2D: TextureKey>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        spine_shader_pool: &SP
    ) {
        mat.init(
            device,
            vec![
                MaterialAttributeDesc {
                    vertex: GeometryDataDesc (SpineVertexDataKindKey::Vertices, EVertexDataFormat::F32),
                    bind: SpineMaterialBlockKindKey::Vertices
                }
            ],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            vec![
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4 },
            ],
            vec![
                SpineMaterialBlockKindKey::Texture
            ],
            &spine_shader_pool.get_spine_shader_colored().bind_group_layouts
        );
    }
}

pub trait TSpineMaterialUpdate<K2D: TextureKey> {
    fn mvp_matrix(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        matrix: Matrix
    ) {
        mat.set_uniform(
            MaterialUniformDesc {
                kind: SpineMaterialBlockKindKey::MVPMatrix,
                format: EUniformDataFormat::Mat4,
            },
            UnifromData::Mat4(matrix)
        );
    }
    fn mask_flag(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        data: Vector4
    ) {
        mat.set_uniform(
            MaterialUniformDesc {
                kind: SpineMaterialBlockKindKey::MaskFlag,
                format: EUniformDataFormat::Float4,
            },
            UnifromData::Float4(data)
        );
    }
}

impl<K2D: TextureKey> TSpineMaterialUpdate<K2D> for Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D> {}