use pi_scene_math::Vector3;



pub struct TargetCamera {
    target: Vector3,
}

impl Default for TargetCamera {
    fn default() -> Self {
        Self { target: Vector3::zeros() }
    }
}

impl TargetCamera {
    pub fn target(&mut self, target: &Vector3) {

    }
}