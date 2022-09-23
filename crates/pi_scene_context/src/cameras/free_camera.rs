use pi_scene_math::Vector3;

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

impl PerspectiveCamera {
    const P0: Vector3 = Vector3::new(-1., -1., 0.);
    const P1: Vector3 = Vector3::new(-1., -1., 0.);
    const P2: Vector3 = Vector3::new(-1., -1., 0.);
    const P3: Vector3 = Vector3::new(-1., -1., 0.);
    const P4: Vector3 = Vector3::new(-1., -1., 0.);
    const P5: Vector3 = Vector3::new(-1., -1., 0.);
    const P6: Vector3 = Vector3::new(-1., -1., 0.);
    const P7: Vector3 = Vector3::new(-1., -1., 0.);
}

pub fn calc_camera_orthographic(orthograhic: &OrthograhicCamera, target: &TargetCamera, node: &TransformNode, camera: &mut Camera) {

}

pub fn calc_camera_perspective(orthograhic: &OrthograhicCamera, target: &TargetCamera, node: &TransformNode, camera: &mut Camera) {
    
}