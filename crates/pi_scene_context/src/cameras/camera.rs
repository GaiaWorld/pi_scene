use pi_scene_math::{vector::{ TMinimizeMaximize }, Vector3, Matrix, Vector4};


pub struct Camera {
    up: Vector3,
    minz: f32,
    maxz: f32,
    /// 
    /// * Define the default inertia of the camera.
    /// * This helps giving a smooth feeling to the camera movement.
    inertia: f32,
    viewport: Vector4,
    view_matrix: Matrix,
    project_matrix: Matrix,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            minz: 0.1,
            maxz: 1000.,
            inertia: 0.7,
            viewport: Vector4::new(0., 0., 1., 1.),
            view_matrix: Matrix::identity(),
            project_matrix: Matrix::identity(),
        }
    }
}

impl Camera {
    pub fn compute(&self, world_matrix: &Matrix, view_matrix: Option<&mut Matrix>, project_matrix: Option<&mut Matrix>) {

    }
}