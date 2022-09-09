use std::hash::Hash;
use pi_scene_geometry::{TVertexDataKindKey, geometry::GeometryDataDesc};

pub trait TMaterialBlockKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {}

pub enum EUniformDataFormat {
    Float,
    Float2,
    Float4,
    Mat2,
    Mat4,
    Texture2D,
}

pub type UniformKindFloat = Option<f32>;
pub type UniformKindFloat2 = Option<(f32, f32)>;
pub type UniformKindFloat4 = Option<(f32, f32, f32, f32)>;
pub type UniformKindMat2 = Option<(f32, f32, f32, f32)>;
pub type UniformKindMat4 = Option<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)>;
// pub type UniformKindTexture2D = Option<(f32, f32, f32, f32)>;

pub struct MaterialAttributeDesc<K: TVertexDataKindKey, K0: TMaterialBlockKindKey> {
    vertex: GeometryDataDesc<K>,
    bind: K0,
}

pub struct Material<K: TVertexDataKindKey, K0: TMaterialBlockKindKey> {
    float_uniforms: Vec<UniformKindFloat>,
    float2_uniforms: Vec<UniformKindFloat2>,
    float4_uniforms: Vec<UniformKindFloat4>,
    mat2_uniforms: Vec<UniformKindMat2>,
    mat4_uniforms: Vec<UniformKindMat4>,
    uniform_slot_index: Vec<usize>,
    uniform_slot_desc: Vec<K0>,
    attributes: Vec<MaterialAttributeDesc<K, K0>>,
    attribute_slot_desc: Vec<K0>,
}

pub struct LightingEnable;

pub struct CastShadow;

pub struct ReceiveShadow;

