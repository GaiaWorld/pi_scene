use pi_hash::XHashMap;
use render_data_container::{TVertexBufferKindKey, TMaterialBlockKindKey, TextureID};
use render_material::{material::Material};
use render_pipeline_key::pipeline_key::PipelineKey;

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
    pub fn update_uniform<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, TID: TextureID>(&self, material_data: &mut Material<VBK, MBKK, TID>) {
        todo!()
    }
}
