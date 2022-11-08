use crate::{Rotation3, Vector3, Number, Matrix, Quaternion, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolRotation}};

#[derive(Debug)]
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
        Self {
            translation: Vector3::zeros(),
            scaling: Vector3::new(1., 1., 1.),
            rotation: Rotation3::identity(),
            local_matrix: Matrix::identity(),
            dirty: false,
        }
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

    pub fn scaling(&self) -> Vector3 {
        self.scaling.clone()
    }

    pub fn scaling_mut(&mut self, rhs: &Vector3) {
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

    pub fn get_rotation_euler_angles(&self, coord: &CoordinateSytem3) -> Vector3 {
        let mut result = Vector3::zeros();
        coord.rotation_matrix_to_euler_angles(&self.rotation, &mut result);
        result
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
    pub fn set_rotation_from_euler_angles(&mut self, x: Number, y: Number, z: Number, coord: &CoordinateSytem3) {
        self.dirty = true;
        CoordinateSytem3::rotation_matrix_mut_euler_angles(x, y, z, &mut self.rotation);
    }
    pub fn matrix(&self) -> &Matrix {
        &self.local_matrix
    }
    pub fn calc_matrix(&mut self) {
        if self.dirty {
            let mut affine = Matrix::identity();
            affine.append_nonuniform_scaling_mut(&self.scaling);
            affine.append_translation_mut(&self.translation);
            affine.mul_to(&self.rotation.to_homogeneous(), &mut self.local_matrix);
            // self.rotation.to_homogeneous().mul_to(&affine, &mut self.local_matrix);
        }

        self.dirty = false;
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use nalgebra::{Transform3 as NTransform3, Matrix3};

    use crate::{coordiante_system::CoordinateSytem3, Rotation3, Vector3, vector::TToolMatrix};

    use super::Transform3;

    #[test]
    fn test_transform() {
        let coord = CoordinateSytem3::default();
        let mut transform = Transform3::default();

        println!("{:?}", transform);

        transform.set_scaling_from_floats(1., 2., 3.);
        transform.set_rotation_from_euler_angles(2., 1.0, -1.0, &coord);
        transform.set_translation_from_floats(10.0, 0.0, 0.0);
        transform.calc_matrix();


        let mut rotation = Rotation3::identity();
        let mut scaling = Vector3::new(1., 1., 1.);
        let mut translation = Vector3::zeros();

        CoordinateSytem3::matrix4_decompose_rotation(transform.matrix(), Some(&mut scaling), Some(&mut rotation), Some(&mut translation));
        
        println!("{:?}", transform.scaling);
        println!("{:?}", scaling);
        assert_relative_eq!(transform.scaling, scaling, epsilon = 0.00001);
        
        println!("{:?}", transform.translation);
        println!("{:?}", translation);
        assert_relative_eq!(transform.translation, translation, epsilon = 0.00001);
        
        println!("{:?}", transform.rotation);
        println!("{:?}", rotation);
        assert_relative_eq!(transform.rotation, rotation, epsilon = 0.00001);

        // let tr = NTransform3::<f32>::from_matrix_unchecked(transform.matrix().clone());
        // let mut mat = transform.matrix().fixed_slice::<3, 3>(0, 0);
        // let mat = Matrix3::copy_from_slice(&mat.sli);
        // let rot = Rotation3::from_matrix_unchecked( );
        // println!("{:?}", rot);
    }
    #[test]
    fn test_transform_right() {
        let coord = CoordinateSytem3::right();
        let mut transform = Transform3::default();

        println!("{:?}", transform);

        transform.set_scaling_from_floats(1., 2., 3.);
        transform.set_rotation_from_euler_angles(2., 1.0, -1.0, &coord);
        transform.set_translation_from_floats(10.0, 0.0, 0.0);
        transform.calc_matrix();


        let mut rotation = Rotation3::identity();
        let mut scaling = Vector3::new(1., 1., 1.);
        let mut translation = Vector3::zeros();

        CoordinateSytem3::matrix4_decompose_rotation(transform.matrix(), Some(&mut scaling), Some(&mut rotation), Some(&mut translation));
        
        println!("{:?}", transform.scaling);
        println!("{:?}", scaling);
        assert_relative_eq!(transform.scaling, scaling, epsilon = 0.00001);
        
        println!("{:?}", transform.translation);
        println!("{:?}", translation);
        assert_relative_eq!(transform.translation, translation, epsilon = 0.00001);
        
        println!("{:?}", transform.rotation);
        println!("{:?}", rotation);
        assert_relative_eq!(transform.rotation, rotation, epsilon = 0.00001);

        // let tr = NTransform3::<f32>::from_matrix_unchecked(transform.matrix().clone());
        // let mut mat = transform.matrix().fixed_slice::<3, 3>(0, 0);
        // let mat = Matrix3::copy_from_slice(&mat.sli);
        // let rot = Rotation3::from_matrix_unchecked( );
        // println!("{:?}", rot);
    }
}