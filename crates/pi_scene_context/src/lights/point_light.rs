use super::ELightFalloffMode;



pub struct PointLight {
    pub falloff: ELightFalloffMode,
    pub intensity: f32,
    pub radius: f32,
}