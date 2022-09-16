use crate::{pipeline::SpinePipelinePool, shaders::SpineShaderPool};


pub struct SceneRenderer {}

impl SceneRenderer {
    pub fn check_colored_shader<SP: SpineShaderPool, SPP: SpinePipelinePool>(
        device: &wgpu::Device,
        shader_pool: &SP,
        pipeline_pool: &mut SPP,
    ) {
        
    }
}