use pi_scene_math::{Vector3, vector::TToolVector3};

use crate::transform::Transform;

use super::camera::Camera;



pub struct TargetCamera {
    pub up: Vector3,
    pub target: Vector3,
}

impl Default for TargetCamera {
    fn default() -> Self {
        Self {
            target: Vector3::zeros(),
            up: Vector3::up(),
        }
    }
}

impl TargetCamera {
    pub fn set_target(&mut self, transform: &Transform, camera: &mut Camera, target: &Vector3) {
        
    }
}