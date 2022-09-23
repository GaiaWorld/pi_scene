use std::hash::Hash;
use pi_scene_data_container::{TVertexBufferKindKey, TMaterialBlockKindKey, TextureID, TexturePool, GeometryBufferPool, TGeometryBufferID};
use pi_scene_geometry::{geometry::{GeometryBufferDesc, Geometry}};
use pi_scene_math::{Matrix, Vector2, Vector4, Matrix2, Color4};
use pi_scene_pipeline_key::uniform_info::calc_uniform_size;

use crate::{error::EMaterialError, texture::MaterialTextureSampler};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EUniformDataFormat {
    Float,
    Float2,
    Float4,
    Color4,
    Mat2,
    Mat4,
}
impl EUniformDataFormat {
    pub fn match_uniform_size(&self) -> wgpu::BufferAddress {
        match self {
            EUniformDataFormat::Float => 4,
            EUniformDataFormat::Float2 => 8,
            EUniformDataFormat::Float4 => 16,
            EUniformDataFormat::Color4 => 16,
            EUniformDataFormat::Mat2 => 16,
            EUniformDataFormat::Mat4 => 64,
        }
    }
    pub fn fill_size(&self) -> wgpu::BufferAddress {
        match self {
            EUniformDataFormat::Float => 4,
            EUniformDataFormat::Float2 => 8,
            EUniformDataFormat::Float4 => 16,
            EUniformDataFormat::Color4 => 16,
            EUniformDataFormat::Mat2 => 16,
            EUniformDataFormat::Mat4 => 16,
        }
    }
}

pub type UniformKindFloat   = f32;
pub type UniformKindFloat2  = Vector2;
pub type UniformKindFloat4  = Vector4;
pub type UniformKindColor4  = Color4;
pub type UniformKindMat2    = Matrix2;
pub type UniformKindMat4    = Matrix;
// pub type UniformKindTexture2D = Option<(f32, f32, f32, f32)>;

pub struct MaterialAttributeDesc<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey> {
    pub vertex: GeometryBufferDesc<VBK>,
    pub bind: MBKK,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialUniformDesc<MBKK: TMaterialBlockKindKey> {
    pub kind: MBKK,
    pub bind: usize,
    pub format: EUniformDataFormat,
}
impl<MBKK: TMaterialBlockKindKey> MaterialUniformDesc<MBKK> {
    pub fn calc_buffer_size(descs: &Vec<MaterialUniformDesc<MBKK>>) -> wgpu::BufferAddress {
        let mut result = 0;
        let mut last_size = 0;
        for desc in descs.iter() {
            let fill_size = desc.format.fill_size();
            if last_size == 0 || last_size == fill_size {
                last_size = fill_size;
            } else if last_size < fill_size {
                last_size = fill_size;
                result += (fill_size - last_size) as wgpu::BufferAddress;
            } else {
                last_size -= fill_size;
            }
            result += desc.format.match_uniform_size() as wgpu::BufferAddress;
        }

        if last_size > 0 {
            result += last_size as wgpu::BufferAddress;
        }

        result
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialTextureDesc<MBKK: TMaterialBlockKindKey> {
    pub kind: MBKK,
    pub bind: u32,
    pub bind_sampler: u32,
}


pub enum UnifromData {
    Float(UniformKindFloat),
    Float2(UniformKindFloat2),
    Float4(UniformKindFloat4),
    Color4(UniformKindColor4),
    Mat2(UniformKindMat2),
    Mat4(UniformKindMat4),
}

impl UnifromData {
    pub fn to_float(&self, data: &mut UniformKindFloat) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Float(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
    pub fn to_float2(&self, data: &mut UniformKindFloat2) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Float2(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
    pub fn to_float4(&self, data: &mut UniformKindFloat4) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Float4(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
    pub fn to_color4(&self, data: &mut UniformKindColor4) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Color4(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
    pub fn to_mat2(&self, data: &mut UniformKindMat2) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Mat2(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
    pub fn to_mat4(&self, data: &mut UniformKindMat4) -> Result<(), EMaterialError> {
        match self {
            UnifromData::Mat4(v) => {
                *data = *v;
                Ok(())
            },
            _ => Err(EMaterialError::UniformDataNotMatch),
        }
    }
}

pub struct Material<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, TID: TextureID> {
    /** Uniforms */
    uniform_bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_descs: Vec<MaterialUniformDesc<MBKK>>,
    /// 每个Uniform在各自数据类型的存储数值中的存储位置
    uniform_type_save_index: Vec<usize>,
    uniform_data_dirty: bool,
    float_pool: Vec<UniformKindFloat>,
    float2_pool: Vec<UniformKindFloat2>,
    float4_pool: Vec<UniformKindFloat4>,
    color4_pool: Vec<UniformKindColor4>,
    mat2_pool: Vec<UniformKindMat2>,
    mat4_pool: Vec<UniformKindMat4>,
    /** Textures */
    texture_bind_group: Option<wgpu::BindGroup>,
    texture_keys: Vec<Option<TID>>,
    texture_samplers: Vec<MaterialTextureSampler>,
    texture_descs: Vec<MaterialTextureDesc<MBKK>>,
    texture_dirty: bool,
    /** Attributes */
    attribute_descs: Vec<GeometryBufferDesc<VBK>>,
    attribute_slot_desc: Vec<MBKK>,
}

impl<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, TID: TextureID> Default for Material<VBK, MBKK, TID> {
    fn default() -> Self {
        Self {
            uniform_bind_group: None,
            uniform_buffer: None,
            uniform_descs: vec![],
            float_pool: vec![],
            float2_pool: vec![],
            float4_pool: vec![],
            color4_pool: vec![],
            mat2_pool: vec![],
            mat4_pool: vec![],
            texture_bind_group: None,
            texture_keys: vec![],
            texture_samplers: vec![],
            texture_descs: vec![],
            texture_dirty: false,
            uniform_type_save_index: vec![],
            attribute_descs: vec![],
            attribute_slot_desc: vec![],
            uniform_data_dirty: false,
        }
    }
}

impl<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, TID: TextureID> Material<VBK, MBKK, TID> {
    pub fn init(
        &mut self,
        device: &wgpu::Device,
        attributes: Vec<GeometryBufferDesc<VBK>>,
        usage: wgpu::BufferUsages,
        uniform_descs: Vec<MaterialUniformDesc<MBKK>>,
        textures: Vec<MaterialTextureDesc<MBKK>>,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        self.uniform_descs.clear();
        self.float_pool.clear();
        self.float2_pool.clear();
        self.float4_pool.clear();
        self.color4_pool.clear();
        self.mat2_pool.clear();
        self.mat4_pool.clear();
        self.texture_keys.clear();
        self.texture_descs.clear();
        self.uniform_type_save_index.clear();
        self.attribute_descs.clear();
        self.attribute_slot_desc.clear();

        let buffer_size = calc_uniform_size(device, MaterialUniformDesc::calc_buffer_size(&uniform_descs) as wgpu::BufferAddress);
        self.uniform_buffer = Some(
            device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: buffer_size,
                usage,
                mapped_at_creation: false,
            })
        );

        for _ in textures.iter() {
            self.texture_keys.push(None);
            self.texture_samplers.push(MaterialTextureSampler::default());
        }
        self.texture_bind_group = None;
        self.texture_descs = textures;

        for desc in uniform_descs.iter() {
            self.add_unifrom_desc(*desc);
        }
        self.attribute_descs = attributes;

        let mut uniform_binds = vec![];
        uniform_binds.push(
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer (
                    wgpu::BufferBinding {
                        buffer: self.uniform_buffer.as_ref().unwrap(),
                        offset: 0,
                        size: wgpu::BufferSize::new(MaterialUniformDesc::calc_buffer_size(&uniform_descs)),
                    }
                ),
            }
        );
        self.uniform_bind_group = Some(
            device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &uniform_bind_group_layout,
                    entries: uniform_binds.as_slice()
                }
            )
        );
    }

    fn add_unifrom_desc(&mut self, desc: MaterialUniformDesc<MBKK>) {
        match desc.format {
            EUniformDataFormat::Float => {
                let index = self.float_pool.len();
                self.float_pool.push(0.);
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
            EUniformDataFormat::Float2 => {
                let index = self.float2_pool.len();
                self.float2_pool.push(Vector2::new(0., 0.));
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
            EUniformDataFormat::Float4 => {
                let index = self.float4_pool.len();
                self.float4_pool.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
            EUniformDataFormat::Color4 => {
                let index = self.float4_pool.len();
                self.float4_pool.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
            EUniformDataFormat::Mat2 => {
                let index = self.float4_pool.len();
                self.float4_pool.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
            EUniformDataFormat::Mat4 => {
                let index = self.mat4_pool.len();
                self.mat4_pool.push(Matrix::identity());
                self.uniform_descs.push(desc);
                self.uniform_type_save_index.push(index);
            },
        }
    }

    pub fn set_uniform(&mut self, desc: MaterialUniformDesc<MBKK>, data: UnifromData) -> Result<(), EMaterialError> {
        match self.uniform_descs.binary_search(&desc) {
            Ok(index) => {
                match self.uniform_type_save_index.get(index) {
                    Some(index) => {
                        match desc.format {
                            EUniformDataFormat::Float => {
                                let value = self.float_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_float(value)
                            },
                            EUniformDataFormat::Float2 => {
                                let value = self.float2_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_float2(value)
                            },
                            EUniformDataFormat::Float4 => {
                                let value = self.float4_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                // println!("################### Float4");
                                data.to_float4(value)
                            },
                            EUniformDataFormat::Color4 => {
                                let value = self.color4_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_color4(value)
                            },
                            EUniformDataFormat::Mat2 => {
                                let value = self.mat2_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_mat2(value)
                            },
                            EUniformDataFormat::Mat4 => {
                                // println!("################### Mat4");
                                let value = self.mat4_pool.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_mat4(value)
                            },
                        }
                    },
                    None => Err(EMaterialError::NotSupportUniformDesc),
                }
            },
            Err(_) => {
                Err(EMaterialError::NotSupportUniformDesc)
            },
        }
    }
    
    pub fn set_texture(&mut self, kind: MaterialTextureDesc<MBKK>, sampler: &MaterialTextureSampler, key: TID) {
        match self.texture_descs.binary_search(&kind) {
            Ok(index) => {
                let new_sampler = sampler.clone();
                let mut bind_dirty = false;
                let old_key = self.texture_keys.get(index).unwrap();
                let old_sampler = self.texture_samplers.get(index).unwrap();
                let old_bind = self.texture_bind_group.as_ref();

                bind_dirty = bind_dirty || (old_key.is_none() || old_key.unwrap() != key);
                bind_dirty = bind_dirty || (!old_sampler.is_same(sampler));
                bind_dirty = bind_dirty || old_bind.is_none();

                self.texture_keys[index] = Some(key);
                self.texture_samplers[index] = new_sampler;

                self.texture_dirty = bind_dirty;
            },
            Err(_) => {
                
            },
        }
    }
    
    pub fn update_uniform<TP: TexturePool<TID>>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, texture_layout: Option<&wgpu::BindGroupLayout>, textures: &TP) {
        if self.texture_dirty {
            match texture_layout {
                Some(texture_layout) => {
                    let mut entries = vec![];
                    let mut i = 0;
                    let keys = &self.texture_keys;
                    let mut samplers = vec![];
                    self.texture_samplers.iter().for_each(|v| { samplers.push(v.to_sampler_resource(device, None)) });
                    for desc in self.texture_descs.iter() {
                        let key = keys.get(i).unwrap();
        
                        match key {
                            Some(key) => {
                                match textures.get(*key) {
                                    Some(textureview) => {
                                        entries.push(
                                            wgpu::BindGroupEntry {
                                                binding: desc.bind_sampler,
                                                resource: wgpu::BindingResource::Sampler(samplers.get(i).unwrap()),
                                            }
                                        );
                                        entries.push(
                                            wgpu::BindGroupEntry {
                                                binding: desc.bind,
                                                resource: wgpu::BindingResource::TextureView (
                                                    textureview
                                                ),
                                            }
                                        );
                                    },
                                    None => {},
                                }
                            },
                            None => {},
                        }
        
                        i += 1;
                    }
                    self.texture_bind_group = Some(
                        device.create_bind_group(
                            &wgpu::BindGroupDescriptor {
                                label: None,
                                layout: texture_layout,
                                entries: entries.as_slice(),
                            }
                        )
                    );
                },
                None => {},
            }
        }
        if self.uniform_data_dirty {
            match self.uniform_buffer.as_ref() {
                Some(buffer) => {
                    let mut datas = vec![];
                    let mut i = 0;
                    let index_list = &self.uniform_type_save_index;
                    for desc in self.uniform_descs.iter() {
                        let index = index_list.get(i).unwrap();
                        match desc.format {
                            EUniformDataFormat::Float => {
                                let data = self.float_pool.get(*index).unwrap();
                                datas.push(*data);
                            },
                            EUniformDataFormat::Float2 => {
                                let data = self.float2_pool.get(*index).unwrap();
                                data.as_slice().iter().for_each(|v| { datas.push(*v); });
                            },
                            EUniformDataFormat::Float4 => {
                                let data = self.float4_pool.get(*index).unwrap();
                                data.as_slice().iter().for_each(|v| { datas.push(*v); });
                            },
                            EUniformDataFormat::Color4 => {
                                let data = self.float4_pool.get(*index).unwrap();
                                data.as_slice().iter().for_each(|v| { datas.push(*v); });
                            },
                            EUniformDataFormat::Mat2 => {
                                let data = self.float4_pool.get(*index).unwrap();
                                data.as_slice().iter().for_each(|v| { datas.push(*v); });
                            },
                            EUniformDataFormat::Mat4 => {
                                let data = self.mat4_pool.get(*index).unwrap();
                                data.as_slice().iter().for_each(|v| { datas.push(*v); });
                            },
                        }
                        i       += 1;
                    }
                    
                    // println!("!!!!!!!!!!!!! {:?}", datas);
                    queue.write_buffer(buffer, 0, bytemuck::cast_slice(&datas));
                    self.uniform_data_dirty = false;
                },
                None => todo!(),
            }
        }
    }
    
    pub fn bind_groups<'a>(&'a self, renderpass: &mut wgpu::RenderPass<'a>) {
        match self.uniform_bind_group.as_ref() {
            Some(bind_group) => {
                renderpass.set_bind_group(0, bind_group, &[]);
            },
            None => todo!(),
        }

        match self.texture_bind_group.as_ref() {
            Some(bind_group) => {
                renderpass.set_bind_group(1, bind_group, &[]);
            },
            None => {},
        }
    }

    pub fn set_vertices<'a, GBID: TGeometryBufferID, GBP: GeometryBufferPool<GBID>>(&'a self, renderpass: &mut wgpu::RenderPass<'a>, geometry: &Geometry<VBK, GBID>, geo_buffer_pool: &GBP) {
        self.attribute_descs.iter().for_each(|desc| {
            let data = geometry.get_vertices(desc);
            match desc.format {
                pi_scene_geometry::vertex_data::EVertexDataFormat::U8 => match data {
                    Some(id) => {
                        match geo_buffer_pool.get_buffer(&id) {
                            Some(buffer) => {
                                renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                },
                pi_scene_geometry::vertex_data::EVertexDataFormat::U16 => match data {
                    Some(id) => {
                        match geo_buffer_pool.get_buffer(&id) {
                            Some(buffer) => {
                                renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                },
                pi_scene_geometry::vertex_data::EVertexDataFormat::U32 => match data {
                    Some(id) => {
                        match geo_buffer_pool.get_buffer(&id) {
                            Some(buffer) => {
                                renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                },
                pi_scene_geometry::vertex_data::EVertexDataFormat::F32 => match data {
                    Some(id) => {
                        match geo_buffer_pool.get_buffer(&id) {
                            Some(buffer) => {
                                renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                },
                pi_scene_geometry::vertex_data::EVertexDataFormat::F64 => match data {
                    Some(id) => {
                        match geo_buffer_pool.get_buffer(&id) {
                            Some(buffer) => {
                                renderpass.set_vertex_buffer(desc.slot, buffer.slice(..));
                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                },
            }
        });

        match geometry.get_indices() {
            Some(id) => match geo_buffer_pool.get_buffer(&id) {
                Some(buffer) => {
                    renderpass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
                    renderpass.draw_indexed(0..geo_buffer_pool.get_size(&id) as u32, 0, 0..);
                },
                None => {
                    
                },
            },
            None => {
                match geometry.get_vertices_number(geo_buffer_pool) {
                    Some(count) => renderpass.draw(0..count as u32, instances),
                    None => todo!(),
                };
            },
        }

    }
    
    pub fn attributes(&self) -> &Vec<GeometryBufferDesc<VBK>> {
        &self.attribute_descs
    }
}

pub struct LightingEnable;

pub struct CastShadow;

pub struct ReceiveShadow;

