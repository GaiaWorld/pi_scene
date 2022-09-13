use crate::transform_node::TransformNode;

use super::{target_camera::TargetCamera, camera::Camera};


pub struct ArcRotateCamera {
    alpha: f32,
    beta: f32,
    gamma: f32,
    radius: f32,
}

pub fn calc_camera_arc_rotate(arc_camera: &ArcRotateCamera, target: &TargetCamera, node: &TransformNode, camera: &mut Camera) {
    
}