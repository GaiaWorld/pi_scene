use pi_scene_math::{Color3};


/// 环境光照明模式
pub enum EAmbientMode {
    None,
    HimesSphere,
    Sphere,
}

/// 环境光
pub struct Ambient {
    pub color: Color3,
    pub intensity: f32,
}

/// 雾效果模式
pub enum EFogMode {
    None,
    Linear,
    Exp2,
}

/// 雾效果参数
pub struct Fog {
    pub mode: EFogMode,
    pub v0: f32,
    pub v1: f32,
    pub v2: f32,
}