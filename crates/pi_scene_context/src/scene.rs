use pi_scene_math::{Matrix, frustum::FrustumPlanes};

use crate::cameras::camera::Camera;


pub struct SceneParam {
    pub transform_matrix: Matrix,
    pub frustum_planes: FrustumPlanes,
}

impl SceneParam {
    pub fn transform_matrix_from_camera(&mut self, camera: &Camera) {
        
    }
    fn update_frustum_planes(&mut self) {

    }
}