///! 数值类型设定在此处限定, 简化拓展和修改 f32/f64

pub mod vector;
pub mod plane;
pub mod frustum;

use nalgebra::{RealField, Vector2 as NVector2, Vector3 as NVector3, Dim, SimdValue, Vector4 as NVector4, UnitQuaternion as NQuaternion, Matrix4 as NMatrix4};

pub type Number = f32;

pub type Vector2 = NVector2<Number>;
pub type Vector3 = NVector3<Number>;
pub type Vector4 = NVector4<Number>;
pub type Color3 = NVector3<Number>;
pub type Color4 = NVector4<Number>;
pub type Quaternion = NQuaternion<Number>;
pub type Matrix = NMatrix4<Number>;


#[cfg(test)]
mod test {
    use crate::{vector::{TMinimizeMaximize, TToolVector3}, Vector3, Matrix};

    #[test]
    fn test_minimize() {
        let v0 = Vector3::new(1., 2., 1.);
        let v1 = Vector3::new(2., 1., 2.);
        let mut v2 = Vector3::new(0., 0., 0.);

        v0.minimize(&v1, &mut v2);

        println!("{:?}", v2);
    }

    #[test]
    fn test_transform() {
        let v0 = Vector3::one();
        let mut transformation = Matrix::identity();
        transformation.append_translation_mut(&Vector3::new(100., 0., 0.));
        transformation.append_scaling_mut(2.);
        println!("{:?}", transformation);

        let mut v1 = Vector3::zeros();
        v0.transform_coordinates(&transformation, &mut v1);
        println!("{:?}", v1);
        let mut n1 = Vector3::zeros();
        v0.transform_normal(&transformation, &mut n1);
        println!("{:?}", n1);
    }
}