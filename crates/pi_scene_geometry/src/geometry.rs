
use std::hash::Hash;
use pi_scene_data_container::{TGeometryBufferID, TVertexBufferKindKey, GeometryBufferPool};
use pi_share::Share;
use crate::error::EGeometryError;
use crate::vertex_data::{VertexBufferU8, VertexBufferU16, VertexBufferU32, VertexBufferF32, VertexBufferF64, EVertexDataFormat};

// #[derive(Debug, Clone)]
// pub struct GeometryKindBuffer<GBID: TGeometryBufferID> {
//     pub u8: Option<GBID>,
//     pub u16: Option<GBID>,
//     pub u32: Option<GBID>,
//     pub f32: Option<GBID>,
//     pub f64: Option<GBID>,
// }

// impl<GBID: TGeometryBufferID> Default for GeometryKindBuffer<GBID> {
//     fn default() -> Self {
//         Self { u8: None, u16: None, u32: None, f32: None, f64: None }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct GeometryDataDesc<VDK: TVertexBufferKindKey>{
//     format: EVertexDataFormat,
//     kind: VDK,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryBufferDesc<VBK: TVertexBufferKindKey>{
    pub slot: u32,
    pub format: EVertexDataFormat,
    pub kind: VBK,
    pub size_per_vertex: usize,
}

pub struct Geometry<VBK: TVertexBufferKindKey, GBID: TGeometryBufferID> {
    u8_buffers: Vec<GBID>,
    u16_buffers: Vec<GBID>,
    u32_buffers: Vec<GBID>,
    f32_buffers: Vec<GBID>,
    f64_buffers: Vec<GBID>,
    indices_buffer: Option<GBID>,
    indices_buffer_u32: Option<GBID>,
    data_descs: Vec<GeometryBufferDesc<VBK>>,
    data_indexs: Vec<usize>,
    position_desc: Option<GeometryBufferDesc<VBK>>,
    instance_descs: Vec<GeometryBufferDesc<VBK>>,
    instance_indexs: Vec<usize>,
}

impl<VBK: TVertexBufferKindKey, GBID: TGeometryBufferID> Geometry<VBK, GBID> {
    pub fn new() -> Self {
        Self {
            u8_buffers: vec![],
            u16_buffers: vec![],
            u32_buffers: vec![],
            f32_buffers: vec![],
            f64_buffers: vec![],
            data_descs: vec![],
            data_indexs: vec![],
            indices_buffer: None,
            indices_buffer_u32: None,
            position_desc: None,
            instance_descs: vec![],
            instance_indexs: vec![],
        }
    }

    pub fn set(&mut self, desc: GeometryBufferDesc<VBK>, data: Option<GBID>) -> Result<(), EGeometryError> {
        if self.data_descs.contains(&desc) == false {
            match desc.format {
                EVertexDataFormat::U8 => {
                    match data {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u8_buffers.len());
                            self.u8_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U16 => {
                    match data {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u16_buffers.len());
                            self.u16_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U32 => {
                    match data {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u32_buffers.len());
                            self.u32_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F32 => {
                    match data {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.f32_buffers.len());
                            self.f32_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F64 => {
                    match data {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.f64_buffers.len());
                            self.f64_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
            }
        } else {
            Err(EGeometryError::GeometryDataCannotSetAgainForTargetDesc)
        }
    }

    pub fn set_instance(&mut self, desc: GeometryBufferDesc<VBK>, data: Option<GBID>) -> Result<(), EGeometryError> {
        if self.instance_descs.contains(&desc) == false {
            match desc.format {
                EVertexDataFormat::U8 => {
                    match data {
                        Some(data) => {
                            self.instance_descs.push(desc);
                            self.instance_indexs.push(self.u8_buffers.len());
                            self.u8_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U16 => {
                    match data {
                        Some(data) => {
                            self.instance_descs.push(desc);
                            self.instance_indexs.push(self.u16_buffers.len());
                            self.u16_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U32 => {
                    match data {
                        Some(data) => {
                            self.instance_descs.push(desc);
                            self.instance_indexs.push(self.u32_buffers.len());
                            self.u32_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F32 => {
                    match data {
                        Some(data) => {
                            self.instance_descs.push(desc);
                            self.instance_indexs.push(self.f32_buffers.len());
                            self.f32_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F64 => {
                    match data {
                        Some(data) => {
                            self.instance_descs.push(desc);
                            self.instance_indexs.push(self.f64_buffers.len());
                            self.f64_buffers.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
            }
        } else {
            Err(EGeometryError::GeometryDataCannotSetAgainForTargetDesc)
        }
    }

    pub fn get_vertices(&self, desc: &GeometryBufferDesc<VBK>) -> Option<GBID> {
        let mut result: Option<GBID> = None;

        match self.data_descs.binary_search(desc) {
            Ok(index) => {
                let index = self.data_indexs[index];
                match desc.format {
                    EVertexDataFormat::U8 => result = match self.u8_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::U16 => result = match self.u16_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::U32 => result = match self.u32_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::F32 => result = match self.f32_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::F64 => result = match self.f64_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                };
                result
            },
            Err(_) => result,
        }
    }

    
    pub fn get_instance(&self, desc: &GeometryBufferDesc<VBK>) -> Option<GBID> {
        let mut result: Option<GBID> = None;

        match self.instance_descs.binary_search(desc) {
            Ok(index) => {
                let index = self.instance_indexs[index];
                match desc.format {
                    EVertexDataFormat::U8 => result = match self.u8_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::U16 => result = match self.u16_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::U32 => result = match self.u32_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::F32 => result = match self.f32_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                    EVertexDataFormat::F64 => result = match self.f64_buffers.get(index) {
                        Some(key) => Some(*key),
                        None => None,
                    },
                };
                result
            },
            Err(_) => result,
        }
    }

    pub fn get_indices(&self) -> Option<GBID> {
        match self.indices_buffer {
            Some(data) => Some(data),
            None => None,
        }
    }
    pub fn record_position_desc(&mut self, desc: GeometryBufferDesc<VBK>) {
        self.position_desc = Some(desc);
    }
    pub fn record_instance_desc(&mut self, desc: GeometryBufferDesc<VBK>) {
        self.position_desc = Some(desc);
    }
    pub fn get_vertices_number<GBP: GeometryBufferPool<GBID>>(&self, pool: &GBP) -> Option<usize> {
        let mut result = match self.position_desc {
            Some(desc) => {
                match self.get_vertices(&desc) {
                    Some(id) => {
                        Some(pool.get_size(&id) / desc.size_per_vertex)
                    },
                    None => None,
                }
            },
            None => None,
        };

        result
    }
}