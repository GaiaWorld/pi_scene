///! 数值类型设定在此处限定, 简化拓展和修改 f32/f64

pub mod vector;

use nalgebra::{RealField, Vector2 as NVector2, Vector3 as NVector3, Dim, SimdValue, Vector4 as NVector4, Quaternion as NQuaternion, Matrix4 as NMatrix4};

pub type Vector2 = NVector2<f32>;
pub type Vector3 = NVector3<f32>;
pub type Vector4 = NVector4<f32>;
pub type Color3 = NVector3<f32>;
pub type Color4 = NVector4<f32>;
pub type Quaternion = NQuaternion<f32>;
pub type Matrix = NMatrix4<f32>;


#[cfg(test)]
mod test {
    use crate::{vector::TMinimizeMaximize, Vector3};

    #[test]
    fn test_minimize() {
        let v0 = Vector3::new(1., 2., 1.);
        let v1 = Vector3::new(2., 1., 2.);
        let mut v2 = Vector3::new(0., 0., 0.);

        v0.minimize(&v1, &mut v2);

        println!("{:?}", v2);
    }
}