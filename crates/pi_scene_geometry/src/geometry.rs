
use std::hash::Hash;
use pi_share::Share;
use crate::TVertexDataKindKey;
use crate::error::EGeometryError;
use crate::vertex_data::{VertexDataU8, VertexDataU16, VertexDataU32, VertexDataF32, VertexDataF64, EVertexDataFormat};

#[derive(Debug, Clone)]
pub struct GeometryKindData {
    pub u8: Option<VertexDataU8>,
    pub u16: Option<VertexDataU16>,
    pub u32: Option<VertexDataU32>,
    pub f32: Option<VertexDataF32>,
    pub f64: Option<VertexDataF64>,
}

impl Default for GeometryKindData {
    fn default() -> Self {
        Self { u8: None, u16: None, u32: None, f32: None, f64: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryDataDesc<K: TVertexDataKindKey>(pub K, pub EVertexDataFormat);

pub struct Geometry<K: TVertexDataKindKey> {
    u8_datas: Vec<VertexDataU8>,
    u16_datas: Vec<VertexDataU16>,
    u32_datas: Vec<VertexDataU32>,
    f32_datas: Vec<VertexDataF32>,
    f64_datas: Vec<VertexDataF64>,
    data_descs: Vec<GeometryDataDesc<K>>,
    data_indexs: Vec<usize>,
}

impl<K: TVertexDataKindKey> Geometry<K> {
    pub fn new() -> Self {
        Self {
            u8_datas: vec![],
            u16_datas: vec![],
            u32_datas: vec![],
            f32_datas: vec![],
            f64_datas: vec![],
            data_descs: vec![],
            data_indexs: vec![],
        }
    }

    pub fn set(&mut self, desc: GeometryDataDesc<K>, data: GeometryKindData) -> Result<(), EGeometryError> {
        if self.data_descs.contains(&desc) == false {
            match desc.1 {
                EVertexDataFormat::U8 => {
                    match data.u8 {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u8_datas.len());
                            self.u8_datas.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U16 => {
                    match data.u16 {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u16_datas.len());
                            self.u16_datas.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::U32 => {
                    match data.u32 {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.u32_datas.len());
                            self.u32_datas.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F32 => {
                    match data.f32 {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.f32_datas.len());
                            self.f32_datas.push(data);
                            Ok(())
                        },
                        None => Err(EGeometryError::GeometryDataSetErrorForTargetFormat),
                    }
                },
                EVertexDataFormat::F64 => {
                    match data.f64 {
                        Some(data) => {
                            self.data_descs.push(desc);
                            self.data_indexs.push(self.f64_datas.len());
                            self.f64_datas.push(data);
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

    pub fn get(&self, desc: GeometryDataDesc<K>) -> GeometryKindData {
        let mut result = GeometryKindData::default();

        match self.data_descs.binary_search(&desc) {
            Ok(index) => {
                let index = self.data_indexs[index];
                match desc.1 {
                    EVertexDataFormat::U8 => result.u8 = Some(self.u8_datas.get(index).unwrap().clone()),
                    EVertexDataFormat::U16 => result.u16 = Some(self.u16_datas.get(index).unwrap().clone()),
                    EVertexDataFormat::U32 => result.u32 = Some(self.u32_datas.get(index).unwrap().clone()),
                    EVertexDataFormat::F32 => result.f32 = Some(self.f32_datas.get(index).unwrap().clone()),
                    EVertexDataFormat::F64 => result.f64 = Some(self.f64_datas.get(index).unwrap().clone()),
                };
                result
            },
            Err(_) => result,
        }
    }
}