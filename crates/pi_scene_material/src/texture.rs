
use std::hash;

use pi_scene_math::Number;
use pi_slotmap::DefaultKey;

pub trait TextureKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + hash::Hash {}

impl TextureKey for DefaultKey {}
impl TextureKey for u8 {}
impl TextureKey for u16 {}
impl TextureKey for u32 {}
impl TextureKey for u64 {}
impl TextureKey for usize {}
impl TextureKey for u128 {}

pub trait TexturePool<K: TextureKey> {
    fn get<'a>(&self, key: K) -> Option<&'a wgpu::TextureView>;
}

pub struct OffsetScaleTexture2D {
    pub u_scale: Number,
    pub v_scale: Number,
    pub u_offset: Number,
    pub v_offset: Number,
}