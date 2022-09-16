use std::hash::Hash;
use pi_scene_geometry::{TVertexDataKindKey, geometry::GeometryDataDesc};
use pi_scene_math::{Number, Matrix, Vector2, Vector4, Matrix2, Color4};
use pi_scene_pipeline_key::uniform_info::calc_uniform_size;
use pi_slotmap::DefaultKey;
use wgpu::util::DeviceExt;

use crate::{texture::{TextureKey, OffsetScaleTexture2D}, error::EMaterialError};

pub trait TMaterialBlockKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {}
impl TMaterialBlockKindKey for DefaultKey {}
impl TMaterialBlockKindKey for u8 {}
impl TMaterialBlockKindKey for u16 {}
impl TMaterialBlockKindKey for u32 {}
impl TMaterialBlockKindKey for u64 {}
impl TMaterialBlockKindKey for u128 {}

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
}

pub type UniformKindFloat   = f32;
pub type UniformKindFloat2  = Vector2;
pub type UniformKindFloat4  = Vector4;
pub type UniformKindColor4  = Color4;
pub type UniformKindMat2    = Matrix2;
pub type UniformKindMat4    = Matrix;
// pub type UniformKindTexture2D = Option<(f32, f32, f32, f32)>;

pub struct MaterialAttributeDesc<K: TVertexDataKindKey, K0: TMaterialBlockKindKey> {
    pub vertex: GeometryDataDesc<K>,
    pub bind: K0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialUniformDesc<K0: TMaterialBlockKindKey> {
    pub kind: K0,
    pub format: EUniformDataFormat,
}
impl<K0: TMaterialBlockKindKey> MaterialUniformDesc<K0> {
    pub fn calc_buffer_size(descs: &Vec<MaterialUniformDesc<K0>>) -> wgpu::BufferAddress {
        let mut result = 0;
        for desc in descs.iter() {
            result += desc.format.match_uniform_size() as wgpu::BufferAddress;
        }

        result
    }
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

pub struct Material<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey> {
    uniform_bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_descs: Vec<MaterialUniformDesc<K0>>,
    uniform_slot_index: Vec<usize>,
    float_uniforms: Vec<UniformKindFloat>,
    float2_uniforms: Vec<UniformKindFloat2>,
    float4_uniforms: Vec<UniformKindFloat4>,
    color4_uniforms: Vec<UniformKindColor4>,
    mat2_uniforms: Vec<UniformKindMat2>,
    mat4_uniforms: Vec<UniformKindMat4>,
    uniform_data_dirty: bool,
    texture_2d_uniforms: Vec<K2D>,
    texture_2d_descs: Vec<K0>,
    attributes: Vec<MaterialAttributeDesc<K, K0>>,
    attribute_slot_desc: Vec<K0>,
}

impl<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey> Default for Material<K, K0, K2D> {
    fn default() -> Self {
        Self {
            uniform_bind_group: None,
            uniform_buffer: None,
            uniform_descs: vec![],
            float_uniforms: vec![],
            float2_uniforms: vec![],
            float4_uniforms: vec![],
            color4_uniforms: vec![],
            mat2_uniforms: vec![],
            mat4_uniforms: vec![],
            texture_2d_uniforms: vec![],
            texture_2d_descs: vec![],
            uniform_slot_index: vec![],
            attributes: vec![],
            attribute_slot_desc: vec![],
            uniform_data_dirty: false,
        }
    }
}

impl<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey> Material<K, K0, K2D> {
    pub fn init(
        &mut self,
        device: &wgpu::Device,
        attributes: Vec<MaterialAttributeDesc<K, K0>>,
        usage: wgpu::BufferUsages,
        uniform_descs: Vec<MaterialUniformDesc<K0>>,
        textures: Vec<K0>,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        self.uniform_descs.clear();
        self.float_uniforms.clear();
        self.float2_uniforms.clear();
        self.float4_uniforms.clear();
        self.color4_uniforms.clear();
        self.mat2_uniforms.clear();
        self.mat4_uniforms.clear();
        self.texture_2d_uniforms.clear();
        self.texture_2d_descs.clear();
        self.uniform_slot_index.clear();
        self.attributes.clear();
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

        self.texture_2d_descs = textures;
        self.attributes = attributes;

        for desc in uniform_descs.iter() {
            self.add_unifrom_desc(*desc);
        }

        let mut binds = vec![];
        binds.push(
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
                    entries: binds.as_slice()
                }
            )
        );
    }
    fn add_unifrom_desc(&mut self, desc: MaterialUniformDesc<K0>) {
        match desc.format {
            EUniformDataFormat::Float => {
                let index = self.float_uniforms.len();
                self.float_uniforms.push(0.);
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
            EUniformDataFormat::Float2 => {
                let index = self.float2_uniforms.len();
                self.float2_uniforms.push(Vector2::new(0., 0.));
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
            EUniformDataFormat::Float4 => {
                let index = self.float4_uniforms.len();
                self.float4_uniforms.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
            EUniformDataFormat::Color4 => {
                let index = self.float4_uniforms.len();
                self.float4_uniforms.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
            EUniformDataFormat::Mat2 => {
                let index = self.float4_uniforms.len();
                self.float4_uniforms.push(Vector4::new(0., 0., 1., 1.));
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
            EUniformDataFormat::Mat4 => {
                let index = self.mat4_uniforms.len();
                self.mat4_uniforms.push(Matrix::identity());
                self.uniform_descs.push(desc);
                self.uniform_slot_index.push(index);
            },
        }
    }
    pub fn set_uniform(&mut self, desc: MaterialUniformDesc<K0>, data: UnifromData) -> Result<(), EMaterialError> {
        match self.uniform_descs.binary_search(&desc) {
            Ok(index) => {
                match self.uniform_slot_index.get(index) {
                    Some(index) => {
                        match desc.format {
                            EUniformDataFormat::Float => {
                                let value = self.float_uniforms.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_float(value)
                            },
                            EUniformDataFormat::Float2 => {
                                let value = self.float2_uniforms.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_float2(value)
                            },
                            EUniformDataFormat::Float4 => {
                                let value = self.float4_uniforms.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                // println!("################### Float4");
                                data.to_float4(value)
                            },
                            EUniformDataFormat::Color4 => {
                                let value = self.color4_uniforms.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_color4(value)
                            },
                            EUniformDataFormat::Mat2 => {
                                let value = self.mat2_uniforms.get_mut(*index).unwrap();
                                self.uniform_data_dirty = true;
                                data.to_mat2(value)
                            },
                            EUniformDataFormat::Mat4 => {
                                // println!("################### Mat4");
                                let value = self.mat4_uniforms.get_mut(*index).unwrap();
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
    pub fn set_texture_2d(&mut self, kind: K0, data: K2D) {

    }
    // pub fn update_uniform_descs(&mut self, uniform_descs: Vec<MaterialUniformDesc<K0>>) {
    //     self.uniform_descs = uniform_descs;
    // }
    pub fn update_uniform(&mut self, queue: &wgpu::Queue) {
        if !self.uniform_data_dirty {
            return;
        }
        match self.uniform_buffer.as_ref() {
            Some(buffer) => {
                let mut datas = vec![];
                let mut i = 0;
                let index_list = &self.uniform_slot_index;
                for desc in self.uniform_descs.iter() {
                    let index = index_list.get(i).unwrap();
                    match desc.format {
                        EUniformDataFormat::Float => {
                            let data = self.float_uniforms.get(*index).unwrap();
                            datas.push(*data);
                        },
                        EUniformDataFormat::Float2 => {
                            let data = self.float2_uniforms.get(*index).unwrap();
                            data.as_slice().iter().for_each(|v| { datas.push(*v); });
                        },
                        EUniformDataFormat::Float4 => {
                            let data = self.float4_uniforms.get(*index).unwrap();
                            data.as_slice().iter().for_each(|v| { datas.push(*v); });
                        },
                        EUniformDataFormat::Color4 => {
                            let data = self.float4_uniforms.get(*index).unwrap();
                            data.as_slice().iter().for_each(|v| { datas.push(*v); });
                        },
                        EUniformDataFormat::Mat2 => {
                            let data = self.float4_uniforms.get(*index).unwrap();
                            data.as_slice().iter().for_each(|v| { datas.push(*v); });
                        },
                        EUniformDataFormat::Mat4 => {
                            let data = self.mat4_uniforms.get(*index).unwrap();
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
    pub fn bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        let mut result = vec![];

        match self.uniform_bind_group.as_ref() {
            Some(bind) => result.push(bind),
            None => todo!(),
        }
        result
    }

    pub fn attributes(&self) -> &Vec<MaterialAttributeDesc<K, K0>> {
        &self.attributes
    }
}

pub struct LightingEnable;

pub struct CastShadow;

pub struct ReceiveShadow;

