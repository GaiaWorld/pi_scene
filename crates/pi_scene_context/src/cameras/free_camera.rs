use crate::transform_node::TransformNode;

use super::{camera::Camera, target_camera::TargetCamera};


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

pub fn calc_camera_orthographic(orthograhic: &OrthograhicCamera, target: &TargetCamera, node: &TransformNode, camera: &mut Camera) {

}

pub fn calc_camera_perspective(orthograhic: &OrthograhicCamera, target: &TargetCamera, node: &TransformNode, camera: &mut Camera) {
    
}