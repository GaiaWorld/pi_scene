///! 数值类型设定在此处限定, 简化拓展和修改 f32/f64

pub mod coordiante_system;
pub mod vector;
pub mod plane;
pub mod frustum;
pub mod transform;
pub mod camera;

use std::ops::Add;

use nalgebra::{Vector2 as NVector2, Vector3 as NVector3, Vector4 as NVector4, UnitQuaternion as NQuaternion,  
    Matrix4 as NMatrix4, SimilarityMatrix3 as NSimilarityMatrix3, Translation3 as NTranslation3,
    Affine3 as NAffine3, Projective3 as NProjective3, Isometry3 as NIsometry3, Rotation3 as NRotation3,
    Matrix2 as NMatrix2, Point3 as NPoint3, Perspective3 as NPerspective3, Orthographic3 as NOrthographic3
};
pub use nalgebra::Quaternion as SQuaternion;

pub type Number = f32;
pub type Vector2 = NVector2<Number>;
pub type Vector3 = NVector3<Number>;
pub type Vector4 = NVector4<Number>;
pub type Color3 = NVector3<Number>;
pub type Color4 = NVector4<Number>;
/// 单位四元数旋转
pub type Quaternion = NQuaternion<Number>;
/// 位移
pub type Translation3 = NTranslation3<Number>;
/// 旋转矩阵
pub type Rotation3 = NRotation3<Number>;
/// 等距变换 - 旋转&位移 - 相机节点
pub type Isometry3 = NIsometry3<Number>;
/// 相似变换 - 旋转&位移&缩放
pub type SimilarityMatrix3 = NSimilarityMatrix3<Number>;
/// 仿射变换
pub type Affine3 = NAffine3<Number>;
/// 投影变换
pub type Projective3 = NProjective3<Number>;
pub type Matrix     = NMatrix4<Number>;
pub type Matrix2    = NMatrix2<Number>;
pub type Point3 = NPoint3<Number>;
pub type Perspective3 = NPerspective3<Number>;
pub type Orthographic3 = NOrthographic3<Number>;
// pub type Transform = NTransform<Number>;

#[cfg(feature = "curve_no_nalgebra")]
pub trait TInterpolateVector {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self;
    fn size() -> usize;
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self;
}

#[cfg(feature = "curve_no_nalgebra")]
impl TInterpolateVector for Vector2 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self {
        let _1 = 1 as KeyFrameCurveValue;
        let _2 = 2 as KeyFrameCurveValue;
        let _3 = 3 as KeyFrameCurveValue;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        return (((value1.scale(part1)) + (value2.scale(part2))) + (tangent1.scale(part3 * frame_delta))) + (tangent2.scale(part4 * frame_delta));
    }
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.clone() + rhs.scale(amount)
    }
    fn size() -> usize {
        8
    }
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs
    }
}

#[cfg(feature = "curve_no_nalgebra")]
impl TInterpolateVector for Vector3 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self {
        let _1 = 1 as KeyFrameCurveValue;
        let _2 = 2 as KeyFrameCurveValue;
        let _3 = 3 as KeyFrameCurveValue;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        return (((value1.scale(part1)) + (value2.scale(part2))) + (tangent1.scale(part3 * frame_delta))) + (tangent2.scale(part4 * frame_delta));
    }
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.clone() + rhs.scale(amount)
    }
    fn size() -> usize {
        12
    }
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs
    }
}

#[cfg(feature = "curve_no_nalgebra")]
impl TInterpolateVector for Vector4 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self {
        let _1 = 1 as KeyFrameCurveValue;
        let _2 = 2 as KeyFrameCurveValue;
        let _3 = 3 as KeyFrameCurveValue;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        return (((value1.scale(part1)) + (value2.scale(part2))) + (tangent1.scale(part3 * frame_delta))) + (tangent2.scale(part4 * frame_delta));
    }
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.clone() + rhs.scale(amount)
    }
    fn size() -> usize {
        16
    }
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs
    }
}
#[cfg(test)]
mod test {

    use crate::{vector::{TMinimizeMaximize, TToolVector3}, Vector3, Matrix, Translation3, SimilarityMatrix3, Affine3, Rotation3, Number, coordiante_system::CoordinateSytem3, transform::Transform3};

    #[test]
    fn test_minimize() {
        let v0 = Vector3::new(1., 2., 1.);
        let v1 = Vector3::new(2., 1., 2.);
        let mut v2 = Vector3::new(0., 0., 0.);

        v0.minimize(&v1, &mut v2);

        // nalgebra::Perspective3::

        println!("{:?}", v2);
    }

    #[test]
    fn test_transform() {
        let coordiante_sys = CoordinateSytem3::default();

        let v0 = Vector3::new(1., 1., 1.);
        let mut transformation = Matrix::identity();
        transformation.append_translation_mut(&Vector3::new(100., 0., 0.));
        transformation.append_scaling_mut(2.);
        println!("{:?}", transformation);

        let mut v1 = Vector3::zeros();
        CoordinateSytem3::transform_coordinates(&v0, &transformation, &mut v1);
        println!("{:?}", v1);
        let mut n1 = Vector3::zeros();
        CoordinateSytem3::transform_normal(&v0, &transformation, &mut n1);
        println!("{:?}", n1);
    }

    #[test]
    fn test_affine() {
        let coordiante_sys = CoordinateSytem3::default();

        let v0 = Vector3::new(1., 1., 1.);
        let mut matrix = Matrix::identity();
        let mut affine = Affine3::identity();
        affine.matrix_mut_unchecked().append_translation_mut(&Vector3::new(100., 0., 0.));
        affine.matrix_mut_unchecked().append_nonuniform_scaling_mut(&Vector3::new(2., 2., 3.));
        
        let mut rotate = Rotation3::from_axis_angle(&Vector3::x_axis(), 3.1415926);
        affine.matrix_mut_unchecked().mul_to(&rotate.to_homogeneous(), &mut matrix);
        println!("{:?}", affine);

        let mut v1 = Vector3::zeros();
        CoordinateSytem3::transform_coordinates(&v0, &matrix, &mut v1);
        println!("{:?}", v1);
        let mut n1 = Vector3::zeros();
        CoordinateSytem3::transform_normal(&v0, &matrix, &mut n1);
        println!("{:?}", n1);
    }
}