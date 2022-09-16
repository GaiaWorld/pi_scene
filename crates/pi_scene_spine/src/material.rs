use std::hash;

use pi_scene_geometry::{TVertexDataKindKey, geometry::GeometryDataDesc, vertex_data::EVertexDataFormat};
use pi_scene_material::{material::{Material, TMaterialBlockKindKey, MaterialUniformDesc, MaterialAttributeDesc, EUniformDataFormat, UnifromData, MaterialTextureDesc, MaterialTextureSampler}, texture::{TextureKey, TexturePool}};
use pi_scene_math::{Matrix, Vector4};

use crate::shaders::{SpineShaderPool, EShader, colored_textured::ColoredTexturedShader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, hash::Hash)]
pub enum SpineVertexDataKindKey {
    Vertices,
}
impl TVertexDataKindKey for SpineVertexDataKindKey {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, hash::Hash)]
pub enum SpineMaterialBlockKindKey{
    Vertices,
    UniformSet0,
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
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4, set: 0, bind: 0 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4, set: 0, bind: 0 },
            ],
            vec![],
            &spine_shader_pool.get_spine_shader_colored().get_uniform_layout()
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
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4, set: 0, bind: 0 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4, set: 0, bind: 0 },
            ],
            vec![
                MaterialTextureDesc { kind: SpineMaterialBlockKindKey::Texture, set: 1, bind: 1, bind_sampler: Some(0) }
                // SpineMaterialBlockKindKey::Texture
            ],
            &spine_shader_pool.get_spine_shader_colored_textured().get_uniform_layout()
        );
    }
    pub fn texture<K2D: TextureKey, TP: TexturePool<K2D>, SP: SpineShaderPool>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        key: K2D,
        textures: &TP,
        spine_shader_pool: &SP,
    ) {
        let kind = MaterialTextureDesc {
            kind: SpineMaterialBlockKindKey::Texture,
            set: 1,
            bind: 1,
            bind_sampler: Some(0),
        };
        let layout = &spine_shader_pool.get_spine_shader_colored_textured().get_texture_layout(&MaterialTextureDesc { kind: SpineMaterialBlockKindKey::Texture, set: 1, bind: 1, bind_sampler: Some(0) });
        match layout {
            Some(layout) => {
                mat.set_texture_2d(
                    device,
                    kind,
                    layout,
                    &MaterialTextureSampler::new(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear),
                    key,
                    textures
                );
            },
            None => {},
        }
    }
}

pub struct SpineMaterialColoredTexturedTwo {}
impl SpineMaterialColoredTexturedTwo {
    pub fn init<SP: SpineShaderPool, K2D: TextureKey>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        spine_shader_pool: &SP,
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
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MVPMatrix, format: EUniformDataFormat::Mat4, set: 0, bind: 0 },
                MaterialUniformDesc { kind: SpineMaterialBlockKindKey::MaskFlag, format: EUniformDataFormat::Float4, set: 0, bind: 0 },
            ],
            vec![
                MaterialTextureDesc { kind: SpineMaterialBlockKindKey::Texture, set: 1, bind: 1, bind_sampler: Some(0) }
            ],
            &spine_shader_pool.get_spine_shader_colored_textured_two().get_uniform_layout()
        );
    }
    pub fn texture<K2D: TextureKey, TP: TexturePool<K2D>, SP: SpineShaderPool>(
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        device: &wgpu::Device,
        key: K2D,
        textures: &TP,
        spine_shader_pool: &SP,
    ) {
        let kind = MaterialTextureDesc {
            kind: SpineMaterialBlockKindKey::Texture,
            set: 1,
            bind: 1,
            bind_sampler: Some(0),
        };
        let layout = &spine_shader_pool.get_spine_shader_colored_textured_two().get_texture_layout(&MaterialTextureDesc { kind: SpineMaterialBlockKindKey::Texture, set: 1, bind: 1, bind_sampler: Some(0) });
        match layout {
            Some(layout) => {
                mat.set_texture_2d(
                    device,
                    kind,
                    layout,
                    &MaterialTextureSampler::new(wgpu::AddressMode::ClampToEdge, wgpu::FilterMode::Linear),
                    key,
                    textures
                );
            },
            None => {},
        }
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
                set: 0,
                bind: 0,
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
                set: 0,
                bind: 0,
            },
            UnifromData::Float4(data)
        );
    }
    fn texture<TP: TexturePool<K2D>, SP: SpineShaderPool>(
        device: &wgpu::Device,
        mat: &mut Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D>,
        shader: EShader,
        shaders: &SP,
        key: K2D,
        textures: &TP,
    ) {
        match shader {
            EShader::Colored => {},
            EShader::ColoredTextured => {
                SpineMaterialColoredTextured::texture(mat, device, key, textures, shaders);
            },
            EShader::TwoColoredTextured => {
                SpineMaterialColoredTexturedTwo::texture(mat, device, key, textures, shaders);
            },
        }
        // mat.set_texture_2d(
        //     device,
        //     kind,
        //     layout,
        //     sampler,
        //     key,
        //     textures
        // );
    }
}

impl<K2D: TextureKey> TSpineMaterialUpdate<K2D> for Material<SpineVertexDataKindKey, SpineMaterialBlockKindKey, K2D> {}