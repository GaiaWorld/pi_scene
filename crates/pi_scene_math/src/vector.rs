use std::ops::IndexMut;

use nalgebra::{RealField, Vector2, Vector3, Matrix, Vector, Dim, SimdValue, Vector4};

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


impl<R: RealField + Copy> TMinimizeMaximize for Vector3<R> {
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