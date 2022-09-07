use std::ops::IndexMut;

use nalgebra::{Translation3, Norm, clamp, AbstractRotation};

use crate::{Number, Vector3, Matrix, Quaternion};

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
    fn up() -> Self;
    fn down() -> Self;
    fn backward() -> Self;
    fn forward() -> Self;
    fn right() -> Self;
    fn left() -> Self;
    fn one() -> Self;
    fn get_angle_between_vectors(&self, v1: &Vector3, normal: &Vector3) -> Number;
    fn length(&self) -> Number;
    fn length_squared(&self) -> Number;
    fn distance(&self, other: &Vector3) -> Number;
    fn distance_squared(&self, other: &Vector3) -> Number;
    fn clamp(&self, min: &Vector3, max: &Vector3, result: &mut Vector3);
    fn transform_coordinates(&self, transformation: &Matrix, result: &mut Vector3);
    fn transform_normal(&self, transformation: &Matrix, result: &mut Vector3);
    fn rotation_from_axis(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Vector3);
    fn rotate_by_quaternion(&self, quaternion: &Quaternion, result: &mut Vector3);
    fn rotate_by_quaternion_around_point(&self, quaternion: &Quaternion, point: Vector3, result: &mut Vector3);
    // fn dot(&self) -> Number;
    // fn cross(&self, rhs: &Self) -> Self;
}

impl TToolVector3 for Vector3 {
    fn up() -> Self {
        Vector3::new(0., 1., 0.)
    }

    fn down() -> Self {
        Vector3::new(0., -1., 0.)
    }

    fn backward() -> Self {
        Vector3::new(0., 0., 1.)
    }

    fn forward() -> Self {
        Vector3::new(0., 0., -1.)
    }

    fn right() -> Self {
        Vector3::new(1., 0., 0.)
    }

    fn left() -> Self {
        Vector3::new(-1., 0., 0.)
    }

    fn one() -> Self {
        Vector3::new(1., 1., 1.)
    }

    fn get_angle_between_vectors(&self, v1: &Vector3, normal: &Vector3) -> Number {
        let n0 = self.normalize();
        let n1 = v1.normalize();
        let dot = n0.dot(&n1);
        let n = n0.cross(&n1);
        
        if n.dot(normal) > 0. {
            dot.acos()
        } else {
            -dot.acos()
        }
    }

    fn length(&self) -> Number {
        self.distance(&Self::zeros())
    }

    fn length_squared(&self) -> Number {
        self.distance_squared(&Self::zeros())
    }

    fn distance(&self, other: &Vector3) -> Number {
        self.metric_distance(other)
    }

    fn distance_squared(&self, other: &Vector3) -> Number {
        let result = self.metric_distance(other);
        result * result
    }

    fn clamp(&self, min: &Vector3, max: &Vector3, result: &mut Vector3) {
        // result.x = self.x.max(min.x).min(max.x);
        // result.y = self.y.max(min.y).min(max.y);
        // result.z = self.z.max(min.z).min(max.z);
        result.copy_from(clamp(self, min, max));
    }

    fn transform_coordinates(&self, transformation: &Matrix, result: &mut Vector3) {
        let mut h = Vector3::to_homogeneous(self);
        h.w = 1.; // coordinate
        transformation.mul_to(&h.clone(), &mut h);

        result.copy_from(&h.xyz());
    }

    fn transform_normal(&self, transformation: &Matrix, result: &mut Vector3) {
        let mut h = Vector3::to_homogeneous(self);
        h.w = 0.; // normal
        transformation.mul_to(&h.clone(), &mut h);

        result.copy_from(&h.xyz());
    }

    fn rotation_from_axis(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Vector3) {
        // todo!()
    }

    fn rotate_by_quaternion(&self, quaternion: &Quaternion, result: &mut Vector3) {
        // todo!()
    }

    fn rotate_by_quaternion_around_point(&self, quaternion: &Quaternion, point: Vector3, result: &mut Vector3) {
        // todo!()
    }
}

pub trait TToolQuaternion {
    fn rotation_quaternion_from_axis(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Quaternion);
    fn to_euler_angles(&self, result: &mut Vector3);
}

pub trait TToolMatrix {
    fn compose(scaling: &Vector3, quaternion: &Quaternion, translation: &Vector3, result: &mut Matrix);
    fn decompose(m: &mut Matrix, scaling: Option<&mut Vector3>, quaternion: Option<&mut Quaternion>, translation: Option<&mut Vector3>);
    fn get_rotation_matrix(&self, result: &mut Matrix);
}