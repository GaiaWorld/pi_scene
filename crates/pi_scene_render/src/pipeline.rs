use render_pipeline_key::pipeline_key::PipelineKey;


pub struct PipelineSimple {
    key: PipelineKey,
    pipeline: wgpu::RenderPipeline,
    pipeline_layout: wgpu::PipelineLayout,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}