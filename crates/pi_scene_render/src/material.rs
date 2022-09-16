use std::sync::Arc;

use pi_hash::XHashMap;
use pi_scene_geometry::TVertexDataKindKey;
use pi_scene_material::{material::{TMaterialBlockKindKey, Material}, texture::TextureKey};
use pi_scene_pipeline_key::pipeline_key::PipelineKey;
use render_core::rhi::shader::Shader;

use crate::pipeline::{PipelineSimple};



pub struct ShaderMaterial {
    // shader: Shader
    vs: wgpu::ShaderModule,
    fs: wgpu::ShaderModule,
    map: XHashMap<PipelineKey, PipelineSimple>,
}

pub type ShaderKey = usize;

pub struct ShaderMaterialPool {
    map: XHashMap<ShaderKey, ShaderMaterial>,
}


pub struct EffectMaterial {
    material: ShaderKey,
    blend_src: wgpu::BlendState,
    blend_dst: wgpu::BlendState,
}

impl EffectMaterial {
    pub fn update_uniform<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey>(&self, material_data: &mut Material<K, K0, K2D>) {
        
    }
}
