

use crate::{Number, Vector3, Matrix, Quaternion, Rotation3, Isometry3};

pub trait TMinimizeMaximize {
    /// 取得两个数据结构中 每个分量的最小值的集合
    fn minimize(&self, rhs: &Self, result: &mut Self);
    /// 取得两个数据结构中 每个分量的最小值的集合
    fn minimize_mut(&mut self, rhs: &Self);
    /// 取得两个数据结构中 每个分量的最大值的集合
    fn maximize(&self, rhs: &Self, result: &mut Self);
    /// 取得两个数据结构中 每个分量的最大值的集合
    fn maximize_mut(&mut self, rhs: &Self);
}


impl TMinimizeMaximize for Vector3 {
    fn minimize(&self, rhs: &Self, result: &mut Self) {
        result.x = self.x.min(rhs.x);
        result.y = self.y.min(rhs.y);
        result.z = self.z.min(rhs.z);
    }

    fn minimize_mut(&mut self, rhs: &Self) {
        self.x = self.x.min(rhs.x);
        self.y = self.y.min(rhs.y);
        self.z = self.z.min(rhs.z);
    }

    fn maximize(&self, rhs: &Self, result: &mut Self) {
        result.x = self.x.max(rhs.x);
        result.y = self.y.max(rhs.y);
        result.z = self.z.max(rhs.z);
    }

    fn maximize_mut(&mut self, rhs: &Self) {
        self.x = self.x.max(rhs.x);
        self.y = self.y.max(rhs.y);
        self.z = self.z.max(rhs.z);
    }
}

pub trait TToolVector3 {
    fn up() -> Vector3;
    fn down() -> Vector3;
    fn backward(&self) -> Vector3;
    fn forward(&self) -> Vector3;
    fn right() -> Vector3;
    fn left() -> Vector3;
    fn one() -> Vector3;
    fn get_angle_between_vectors(v0: &Vector3, v1: &Vector3, normal: &Vector3) -> Number;
    fn length(v0: &Vector3) -> Number;
    fn length_squared(v0: &Vector3) -> Number;
    fn distance(v0: &Vector3, other: &Vector3) -> Number;
    fn distance_squared(v0: &Vector3, other: &Vector3) -> Number;
    fn clamp(v0: &Vector3, min: &Vector3, max: &Vector3, result: &mut Vector3);
    fn transform_coordinates(v0: &Vector3, transformation: &Matrix, result: &mut Vector3);
    fn transform_coordinates_floats(x: Number, y: Number, z: Number, transformation: &Matrix, result: &mut Vector3);
    fn transform_normal(v0: &Vector3, transformation: &Matrix, result: &mut Vector3);
    fn transform_normal_floats(x: Number, y: Number, z: Number, transformation: &Matrix, result: &mut Vector3);
    fn rotation_from_axis(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Vector3);
    fn rotate_by_quaternion(v0: &Vector3, quaternion: &Quaternion, result: &mut Vector3);
    fn rotate_by_quaternion_around_point(v0: &Vector3, quaternion: &Quaternion, point: Vector3, result: &mut Vector3);
    // fn dot(&self) -> Number;
    // fn cross(&self, rhs: &Self) -> Self;
}



pub trait TToolRotation {
    /// * `x` Pitch
    /// * `y` Yaw
    /// * `z` Roll
    fn quaternion_from_euler_angles(x: Number, y: Number, z: Number) -> Quaternion;
    fn quaternion_mut_euler_angles(x: Number, y: Number, z: Number, result: &mut Quaternion);
    fn quaternion_from_unit_vector(axis: &nalgebra::Unit<Vector3>, vec_to: &Vector3) -> Quaternion;
    
    /// * `x` Pitch
    /// * `y` Yaw
    /// * `z` Roll
    fn quaternion_mut_yaw_pitch_roll(&self, yaw: Number, pitch: Number, roll: Number, result: &mut Quaternion);
    fn quaternion_mut_axis(&self, axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Quaternion);
    fn quaternion_to_euler_angles(&self, rotation: &Quaternion, result: &mut Vector3);
    fn rotation_matrix_from_euler_angles_toref(x: Number, y: Number, z: Number, result: &mut Rotation3);
    fn rotation_matrix_from_euler_angles(x: Number, y: Number, z: Number) -> Rotation3;
    fn rotation_matrix_mut_euler_angles(x: Number, y: Number, z: Number, result: &mut Rotation3);
    fn rotation_matrix_from_axises(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3) -> Rotation3;
    fn quaternion_from_axis_angle(axis1: &Vector3, radian: Number) -> Quaternion;
    
    /// * `x` Pitch
    /// * `y` Yaw
    /// * `z` Roll
    fn rotation_matrix_mut_yaw_pitch_roll(&self, yaw: Number, pitch: Number, roll: Number, result: &mut Rotation3);
    fn rotation_matrix_mut_axis(&self, axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Rotation3);
    fn rotation_matrix_to_euler_angles(&self, rotation: &Rotation3, result: &mut Vector3);
}

pub trait TToolMatrix {
    fn try_inverse_mut(matrix: &mut Matrix) -> bool;
    fn mul_to(a: &Matrix, b: &Matrix, y: & mut Matrix);
    fn matrix4_mul_matrix4(a: &Matrix, b: &Matrix, y: & mut Matrix);
    fn matrix4_mul_vector4(a: &Matrix, b: &crate::Vector4, y: & mut crate::Vector4);
    fn matrix4_from_xyz_axes(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Matrix);
    fn matrix4_decompose(m: &Matrix, scaling: Option<&mut Vector3>, quaternion: Option<&mut Quaternion>, translation: Option<&mut Vector3>) -> bool;
    fn matrix4_decompose_rotation(m: &Matrix, scaling: Option<&mut Vector3>, rotation: Option<&mut Rotation3>, translation: Option<&mut Vector3>) -> bool;
    fn matrix4_compose(scaling: &Vector3, quaternion: &Quaternion, translation: &Vector3, result: &mut Matrix);
    fn matrix4_compose_euler_angle(scaling: &Vector3, eulers: &Vector3, translation: &Vector3, result: &mut Matrix);
    fn matrix4_compose_rotation(scaling: &Vector3, rotmat: &Rotation3, translation: &Vector3, result: &mut Matrix);
    fn matrix4_compose_no_rotation(scaling: &Vector3, translation: &Vector3, result: &mut Matrix);
    fn matrix4_compose_quaternion(scaling: &Vector3, quaternion: &Quaternion, translation: &Vector3, result: &mut Matrix);
    fn rotation_align_to(from: &Vector3, to: &Vector3, result: &mut Matrix);
    fn lookat(&self, eye: &Vector3, target: &Vector3, up: &Vector3, result: &mut Isometry3);
}