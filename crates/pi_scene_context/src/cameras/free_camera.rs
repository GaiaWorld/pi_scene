
pub struct OrthograhicCamera {
    orth_left: f32,
    orth_top: f32,
    orth_right: f32,
    orth_bottom: f32,
}

pub enum EFovMode {
    VerticalFixed,
    HorizontalFixed
}

pub struct PerspectiveCamera {
    fov: f32,
    fov_mode: EFovMode,
}