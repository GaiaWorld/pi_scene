use std::sync::Arc;

pub enum EVertexDataFormat {
    Float,
    Int,
    U16,
}

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

pub struct VertexData {
    pub kind: EVertexDataKind,
    pub data: Arc<[u8]>,
    pub offset: u32,
    pub size: u32,
}