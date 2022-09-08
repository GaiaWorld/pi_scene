use pi_scene_math::{Vector3, Quaternion, Matrix, Number, Rotation3, Affine3, transform::Transform3};

pub struct TransformNode {
    /// 变换
    transform: Transform3,
    /// 全局变换矩阵
    world_matrix: Matrix,
}

impl Default for TransformNode {
    fn default() -> Self {
        Self {
            world_matrix: Matrix::identity(),
            transform: Transform3::identity(),
        }
    }
}

impl TransformNode {
    pub fn local_matrix(&self) -> &Matrix {
        self.transform.matrix()
    }
    pub fn calc_local_matrix(&mut self) {
        self.transform.calc_matrix();
    }
    pub fn world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }
    pub fn calc_world_matrix(&mut self, parent: Option<&Self>) {
        self.transform.calc_matrix();

        match parent {
            Some(parent) => {
                parent.world_matrix.mul_to(self.transform.matrix(), &mut self.world_matrix);
            },
            None => {},
        }
    }
}