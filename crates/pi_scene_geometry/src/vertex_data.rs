use std::{sync::Arc, borrow::BorrowMut};

use pi_share::Share;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EVertexDataFormat {
    U8,
    U16,
    U32,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy)]
pub enum EVertexDataKind {
    Position,
    Position2D,
    ColorKind,
    UV,
    Normal,
    Tangent,
    MatricesIndicesKind,
    MatricesWeightsKind,
    MatricesIndicesExtraKind,
    MatricesWeightsExtraKind,
    UV2,
    UV3,
    UV4,
    UV5,
    UV6,
    UV7,
    UV8,
    UV9,
    UV10,
    UV11,
    UV12,
    UV13,
    UV14,
    UV15,
    UV16,
}

#[derive(Debug, Clone)]
pub struct VertexDataU8 {
    pub kind: EVertexDataKind,
    pub data: Share<Vec<u8>>,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct VertexDataU16 {
    pub kind: EVertexDataKind,
    pub data: Share<Vec<u16>>,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct VertexDataU32 {
    pub kind: EVertexDataKind,
    pub data: Share<Vec<u32>>,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct VertexDataF32 {
    pub kind: EVertexDataKind,
    pub data: Share<Vec<f32>>,
    pub offset: u32,
    pub size: u32,
}

impl VertexDataF32 {
    pub fn set(&mut self, data: &[f32], offset: usize) {
        // let len = data.len();
        // let end = len + offset;
        // let store = self.data.borrow_mut();

        // for i in 0..len {
        //     let t = store.get_mut(i + offset).unwrap();
        //     *t = data[i];
        // }
    }
}

#[derive(Debug, Clone)]
pub struct VertexDataF64 {
    pub kind: EVertexDataKind,
    pub data: Share<Vec<f64>>,
    pub offset: u32,
    pub size: u32,
}