use pi_scene_math::{vector::{ TMinimizeMaximize }, Vector3, Matrix, Vector4};


// 初始方向为z轴正方向
pub struct Camera {
    pub up: Vector3,
    pub fov: f32,
    pub minz: f32,
    pub maxz: f32,
    /// 
    /// * Define the default inertia of the camera.
    /// * This helps giving a smooth feeling to the camera movement.
    pub inertia: f32,
    pub viewport: Vector4,
    pub view_matrix: Matrix,
    pub project_matrix: Matrix,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            fov: 0.75,
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