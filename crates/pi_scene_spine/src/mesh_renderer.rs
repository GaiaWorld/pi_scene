use pi_scene_material::{texture::{TextureKey, TexturePool}, material::{Material, UniformKindFloat4, UniformKindMat4}};
use pi_scene_math::{Number, Matrix, Vector4};
use pi_scene_pipeline_key::pipeline_key::PipelineKey;
use wgpu::DepthStencilState;

use crate::{mesh::Mesh, shaders::{EShader, SpineShaderPool}, pipeline::{SpinePipelinePool, SpinePipeline}};


pub struct  MeshRenderer<K2D: TextureKey> {
    mesh: Mesh<K2D>,
    blend: Option<wgpu::BlendState>,
    shader: EShader,
    pipeline_key: Option<PipelineKey>,
}

impl<K2D: TextureKey> MeshRenderer<K2D> {
    pub fn new() -> Self {
        Self {
            mesh: Mesh::new(),
            blend: None,
            shader: EShader::Colored,
            pipeline_key: None,
        }
    }
    pub fn update<SP: SpineShaderPool, SPP: SpinePipelinePool, TP: TexturePool<K2D>>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Number],
        indices: &[u16],
        shader: EShader,
        mvp: UniformKindMat4,
        mask_flag: UniformKindFloat4,
        src_factor: wgpu::BlendFactor,
        dst_factor: wgpu::BlendFactor,
        target_format: wgpu::TextureFormat,
        depth_stencil: Option<DepthStencilState>,
        texture_key: Option<K2D>,
        shaders: &mut SP,
        pipelines: &mut SPP,
        textures: &TP,
    ) {
        self.mesh.init(device, shader, shaders);
        self.mesh.mvp_matrix(queue, mvp);
        self.mesh.mask_flag(queue, mask_flag);
        self.mesh.texture(device, texture_key, shaders, textures);
        self.mesh.set_vertices(device, queue, vertices);
        self.mesh.set_indices(device, queue, indices);
        self.shader = shader;
        self.blend = Some(
            wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor,
                    dst_factor,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent::OVER,
            }
        );
        let targets = [
            wgpu::ColorTargetState {
                format: target_format,
                blend: self.blend,
                write_mask: wgpu::ColorWrites::ALL,
            }
        ];
        self.pipeline_key = Some(SpinePipeline::check(shader, device, shaders, pipelines, &targets, wgpu::PrimitiveState::default(), depth_stencil));
    }
    pub fn draw<'a, SPP: SpinePipelinePool>(
        &'a self,
        queue: &wgpu::Queue,
        renderpass: &mut wgpu::RenderPass<'a>,
        pipelines: &'a SPP,
    ) {
        match self.pipeline_key {
            Some(key) => {
                // println!(">>>>>>>>>>>>>>>> {}", key);
                match SpinePipeline::get(self.shader, pipelines, key) {
                    Some(pipeline) => {
                        renderpass.set_pipeline(&pipeline.pipeline);
                        self.mesh.draw(queue, renderpass);
                    },
                    None => {},
                }
            },
            None => {
                
            },
        }
    }
}

pub struct MeshRendererPool<K2D: TextureKey> {
    renderers: Vec<MeshRenderer<K2D>>,
    counter: usize,
}

impl<K2D: TextureKey> Default for  MeshRendererPool<K2D> {
    fn default() -> Self {
        Self { renderers: vec![], counter: 0 }
    }
}

impl<K2D: TextureKey>  MeshRendererPool<K2D> {
    pub fn insert<SP: SpineShaderPool, SPP: SpinePipelinePool, TP: TexturePool<K2D>>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Number],
        indices: &[u16],
        shader: EShader,
        mvp: UniformKindMat4,
        mask_flag: UniformKindFloat4,
        src_factor: wgpu::BlendFactor,
        dst_factor: wgpu::BlendFactor,
        target_format: wgpu::TextureFormat,
        depth_stencil: Option<DepthStencilState>,
        texture_key: Option<K2D>,
        shaders: &mut SP,
        pipelines: &mut SPP,
        textures: &TP,
    ) {
        self.counter += 1;
        if self.renderers.len() < self.counter {
            let renderer = MeshRenderer::new();
            self.renderers.push(renderer);
        }

        self.renderers.get_mut(self.counter - 1).unwrap().update(device, queue, vertices, indices, shader, mvp, mask_flag, src_factor, dst_factor, target_format, depth_stencil, texture_key, shaders, pipelines, textures);
    }
    pub fn reset(&mut self) {
        self.counter = 0;
    }
    pub fn draw<'a, SPP: SpinePipelinePool>(
        &'a self,
        queue: &wgpu::Queue,
        renderpass: &mut wgpu::RenderPass<'a>,
        pipelines: &'a SPP,
    ) {
        for i in 0.. self.counter {
            let renderer = self.renderers.get(i).unwrap();
            renderer.draw(queue, renderpass, pipelines);
        }
    }
}