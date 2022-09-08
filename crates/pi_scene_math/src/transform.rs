use crate::{Rotation3, Vector3, Number, Matrix, Quaternion, coordiante_system::CoordinateSytem3, vector::TToolMatrix};


pub struct Transform3 {
    /// 位移
    translation: Vector3,
    /// 缩放
    scaling: Vector3,
    /// 旋转矩阵
    rotation: Rotation3,
    /// 仿射矩阵
    local_matrix: Matrix,
    dirty: bool,
}

impl Default for Transform3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform3 {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translation(&self) -> Vector3 {
        self.translation.clone()
    }

    pub fn translation_mut(&mut self, rhs: &Vector3) {
        self.dirty = true;
        self.translation.copy_from(rhs);
    }

    pub fn set_translation_from_floats(&mut self, x: f32, y: f32, z: f32) {
        self.dirty = true;
        self.translation.x = x;self.translation.y = y;self.translation.z = z;
    }

    pub fn get_scaling(&self) -> Vector3 {
        self.scaling.clone()
    }

    pub fn set_scaling(&mut self, rhs: &Vector3) {
        self.dirty = true;
        self.scaling.copy_from(rhs);
    }

    pub fn set_scaling_from_floats(&mut self, x: f32, y: f32, z: f32) {
        self.dirty = true;
        self.scaling.x = x;self.scaling.y = y;self.scaling.z = z;
    }

    pub fn set_scaling_uniform(&mut self, s: f32) {
        self.dirty = true;
        self.scaling.x = s;self.scaling.y = s;self.scaling.z = s;
    }

    pub fn get_rotation_euler_angles(&self) -> Vector3 {
        let (z, x, y) = self.rotation.euler_angles();
        Vector3::new(x, y, z)
    }

    pub fn rotation_matrix(&self) -> Rotation3 {
        self.rotation.clone()
    }
    pub fn rotation_matrix_mut(&mut self, rhs: &Rotation3) {
        self.dirty = true;
        self.rotation.clone_from(rhs);
    }
    pub fn rotation_quaternion(&self) -> Quaternion {
        Quaternion::from_rotation_matrix(&self.rotation)
    }
    pub fn rotation_quaternion_mut(&mut self, rhs: &Quaternion) {
        self.dirty = true;
        self.rotation.clone_from(&rhs.to_rotation_matrix());
    }
    pub fn set_rotation_from_euler_angles(&mut self, x: Number, y: Number, z: Number) {
        self.dirty = true;
        self.rotation.clone_from(&Rotation3::from_euler_angles(z, x, y));
    }
    pub fn matrix(&self) -> &Matrix {
        &self.local_matrix
    }
    pub fn calc_matrix(&mut self) {
        if self.dirty {
            let mut affine = Matrix::identity();
            affine.append_nonuniform_scaling_mut(&self.scaling);
            affine.append_translation_mut(&self.scaling);
            affine.mul_to(&self.rotation.to_homogeneous(), &mut self.local_matrix);
        }

        self.dirty = false;
    }
}