use super::ELightFalloffMode;


pub struct SpotLight {
    pub falloff: ELightFalloffMode,
    pub intensity: f32,
    pub radius: f32,
    pub angle: f32,
    pub inner_angle: f32,
}