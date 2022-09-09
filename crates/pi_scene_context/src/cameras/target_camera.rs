use pi_scene_math::{Vector3, vector::TToolVector3, coordiante_system::CoordinateSytem3};

use crate::transform_node::TransformNode;

use super::camera::Camera;



pub struct TargetCamera {
    pub up: Vector3,
    pub target: Vector3,
}

impl Default for TargetCamera {
    fn default() -> Self {
        Self {
            target: Vector3::zeros(),
            up: CoordinateSytem3::up(),
        }
    }
}

impl TargetCamera {
    pub fn set_target(&mut self, transform: &TransformNode, camera: &mut Camera, target: &Vector3) {
        
    }
}