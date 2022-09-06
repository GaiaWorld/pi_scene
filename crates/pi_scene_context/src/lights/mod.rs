
pub enum ELightFalloffMode {
    None,
    Phong,
    Physic,
    GLTF,
}

pub mod light;
pub mod direction_light;
pub mod point_light;
pub mod spot_light;
pub mod shadow;