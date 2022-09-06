use pi_scene_math::{Vector3, Quaternion, Matrix};



pub struct Transform {
    translation: Vector3,
    rotation: Vector3,
    scaling: Vector3,
    rotate_quaternion: Quaternion,
    local_matrix: Matrix,
    world_matrix: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scaling: Vector3::new(1., 1., 1.),
            rotate_quaternion: Quaternion::identity(),
            local_matrix: Matrix::identity(),
            world_matrix: Matrix::identity(),
        }
    }
}

impl Transform {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn get_translation(&self) -> Vector3 {
        self.translation.clone()
    }

    pub fn set_translation(&mut self, rhs: &Vector3) {

    }

    pub fn set_translation_from_floats(&mut self, x: f32, y: f32, z: f32) {

    }

    pub fn get_scaling(&self) -> Vector3 {
        self.scaling.clone()
    }

    pub fn set_scaling(&mut self, rhs: &Vector3) {

    }

    pub fn set_scaling_from_floats(&mut self, x: f32, y: f32, z: f32) {

    }

    pub fn set_scaling_from_float(&mut self, s: f32) {

    }

    pub fn get_rotation_euler_angles(&self) -> Vector3 {
        self.rotation.clone()
    }

    pub fn get_rotation_quaternion(&self) -> Quaternion {
        self.rotate_quaternion.clone()
    }

    pub fn set_rotation_quaternion(&mut self, rhs: &Quaternion) {
        self.rotate_quaternion.clone_from(rhs);
        // change rotation
    }

    pub fn set_rotation_from_euler_angles(&mut self, x: f32, y: f32, z: f32) {
        
    }

    pub fn set_rotation_from_quaternion_floats(&mut self, x: f32, y: f32, z: f32, w: f32) {

    }

    pub fn compose_to_matrix(&self, m: &mut Matrix) {
        
    }

    pub fn decompose_from_matrix(&mut self, m: &Matrix) {
        
    }
}