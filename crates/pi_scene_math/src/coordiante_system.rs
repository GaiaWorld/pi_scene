use core::num;

use nalgebra::{clamp, Matrix3, RawStorage, RawStorageMut, SimdComplexField};

use crate::{vector::{TToolVector3, TToolMatrix, TToolRotation}, Vector3, Number, Matrix, Quaternion, Rotation3, Vector4, Point3, Isometry3};


#[derive(Debug, Clone, Copy)]
pub enum ECoordinateSytem3 {
    Left,
    Right,
}

pub struct CoordinateSytem3 {
    mode: ECoordinateSytem3,
}

impl Default for CoordinateSytem3 {
    fn default() -> Self {
        Self::left()
    }
}

impl CoordinateSytem3 {
    pub fn left() -> Self {
        Self { mode: ECoordinateSytem3::Left }
    }
    pub fn right() -> Self {
        Self { mode: ECoordinateSytem3::Right }
    }
    pub fn mode(& self) -> ECoordinateSytem3 {
        self.mode 
    }
}

impl TToolVector3 for CoordinateSytem3 {
    fn up() -> Vector3 {
        Vector3::new(0., 1., 0.)
    }

    fn down() -> Vector3 {
        Vector3::new(0., -1., 0.)
    }

    fn backward(&self) -> Vector3 {
        match self.mode {
            ECoordinateSytem3::Left => Vector3::new(0., 0., -1.),
            ECoordinateSytem3::Right => Vector3::new(0., 0., 1.),
        }
    }

    fn forward(&self) -> Vector3 {
        match self.mode {
            ECoordinateSytem3::Left => Vector3::new(0., 0., 1.),
            ECoordinateSytem3::Right => Vector3::new(0., 0., -1.),
        }
    }

    fn right() -> Vector3 {
        Vector3::new(1., 0., 0.)
    }

    fn left() -> Vector3 {
        Vector3::new(-1., 0., 0.)
    }

    fn one() -> Vector3 {
        Vector3::new(1., 1., 1.)
    }

    fn get_angle_between_vectors(v0: &Vector3, v1: &Vector3, normal: &Vector3) -> Number {
        let n0 = v0.normalize();
        let n1 = v1.normalize();
        let dot = n0.dot(&n1);
        let n = n0.cross(&n1);
        
        if n.dot(normal) > 0. {
            dot.acos()
        } else {
            -dot.acos()
        }
    }

    fn length(v0: &Vector3) -> Number {
        v0.metric_distance(&Vector3::zeros())
    }

    fn length_squared(v0: &Vector3) -> Number {
        let result = v0.metric_distance(&Vector3::zeros());
        result * result
    }

    fn distance(v0: &Vector3, other: &Vector3) -> Number {
        v0.metric_distance(other)
    }

    fn distance_squared(v0: &Vector3, other: &Vector3) -> Number {
        let result = v0.metric_distance(other);
        result * result
    }

    fn clamp(v0: &Vector3, min: &Vector3, max: &Vector3, result: &mut Vector3) {
        // result.x = self.x.max(min.x).min(max.x);
        // result.y = self.y.max(min.y).min(max.y);
        // result.z = self.z.max(min.z).min(max.z);
        result.copy_from(clamp(v0, min, max));
    }

    fn transform_coordinates(v0: &Vector3, transformation: &Matrix, result: &mut Vector3) {
        // let mut h = Vector3::to_homogeneous(v0);
        // h.w = 1.; // coordinate
        // CoordinateSytem3::matrix4_mul_vector4(transformation, &h.clone(), &mut h);
        // // transformation.mul_to(&h.clone(), &mut h);
        // result.copy_from(&h.xyz());

        let m = transformation.as_slice();
        let x = v0.x; let y = v0.y; let z = v0.z;
        let rx = x * m[0] + y * m[4] + z * m[8] + m[12];
        let ry = x * m[1] + y * m[5] + z * m[9] + m[13];
        let rz = x * m[2] + y * m[6] + z * m[10] + m[14];
        let rw = 1. / (x * m[3] + y * m[7] + z * m[11] + m[15]);
        result.x = rx * rw;
        result.y = ry * rw;
        result.z = rz * rw;

    }

    fn transform_coordinates_floats(x: Number, y: Number, z: Number, transformation: &Matrix, result: &mut Vector3) {
        // let mut h = Vector3::to_homogeneous(v0);
        // h.w = 1.; // coordinate
        // CoordinateSytem3::matrix4_mul_vector4(transformation, &h.clone(), &mut h);
        // // transformation.mul_to(&h.clone(), &mut h);
        // result.copy_from(&h.xyz());

        let m = transformation.as_slice();
        let rx = x * m[0] + y * m[4] + z * m[8] + m[12];
        let ry = x * m[1] + y * m[5] + z * m[9] + m[13];
        let rz = x * m[2] + y * m[6] + z * m[10] + m[14];
        let rw = 1. / (x * m[3] + y * m[7] + z * m[11] + m[15]);
        result.x = rx * rw;
        result.y = ry * rw;
        result.z = rz * rw;

    }

    fn transform_normal(v0: &Vector3, transformation: &Matrix, result: &mut Vector3) {
        // let mut h = Vector3::to_homogeneous(v0);
        // h.w = 0.; // normal
        // CoordinateSytem3::matrix4_mul_vector4(transformation, &h.clone(), &mut h);
        // // transformation.mul_to(&h.clone(), &mut h);
        // result.copy_from(&h.xyz());

        let m = transformation.as_slice();
        result.x = v0.x * m[0] + v0.y * m[4] + v0.z * m[8];
        result.y = v0.x * m[1] + v0.y * m[5] + v0.z * m[9];
        result.z = v0.x * m[2] + v0.y * m[6] + v0.z * m[10];
    }

    fn transform_normal_floats(x: Number, y: Number, z: Number, transformation: &Matrix, result: &mut Vector3) {
        // let mut h = Vector3::to_homogeneous(v0);
        // h.w = 0.; // normal
        // CoordinateSytem3::matrix4_mul_vector4(transformation, &h.clone(), &mut h);
        // // transformation.mul_to(&h.clone(), &mut h);
        // result.copy_from(&h.xyz());

        let m = transformation.as_slice();
        result.x = x * m[0] + y * m[4] + z * m[8];
        result.y = x * m[1] + y * m[5] + z * m[9];
        result.z = x * m[2] + y * m[6] + z * m[10];
    }

    fn rotation_from_axis(_axis1: &Vector3, _axis2: &Vector3, _axis3: &Vector3, _result: &mut Vector3) {
        // todo!()
    }

    fn rotate_by_quaternion(_v0: &Vector3, _quaternion: &Quaternion, _result: &mut Vector3) {
        // todo!()
    }

    fn rotate_by_quaternion_around_point(_v0: &Vector3, _quaternion: &Quaternion, _point: Vector3, _result: &mut Vector3) {
        // todo!()
    }
}


impl TToolMatrix for CoordinateSytem3 {
    fn try_inverse_mut(matrix: &mut Matrix) -> bool {
        // matrix.try_inverse_mut()
        let m = matrix.as_mut_slice();
        let m00 = m[ 0]; let m01 = m[ 1]; let m02 = m[ 2]; let m03 = m[ 3];
        let m10 = m[ 4]; let m11 = m[ 5]; let m12 = m[ 6]; let m13 = m[ 7];
        let m20 = m[ 8]; let m21 = m[ 9]; let m22 = m[10]; let m23 = m[11];
        let m30 = m[12]; let m31 = m[13]; let m32 = m[14]; let m33 = m[15];
        let det_22_33 = m22 * m33 - m32 * m23;
        let det_21_33 = m21 * m33 - m31 * m23;
        let det_21_32 = m21 * m32 - m31 * m22;
        let det_20_33 = m20 * m33 - m30 * m23;
        let det_20_32 = m20 * m32 - m22 * m30;
        let det_20_31 = m20 * m31 - m30 * m21;
        let cofact_00 = 0. + (m11 * det_22_33 - m12 * det_21_33 + m13 * det_21_32);
        let cofact_01 = 0. - (m10 * det_22_33 - m12 * det_20_33 + m13 * det_20_32);
        let cofact_02 = 0. + (m10 * det_21_33 - m11 * det_20_33 + m13 * det_20_31);
        let cofact_03 = 0. - (m10 * det_21_32 - m11 * det_20_32 + m12 * det_20_31);
        let det = m00 * cofact_00 + m01 * cofact_01 + m02 * cofact_02 + m03 * cofact_03;
        if det == 0. {
            // not invertible
            // other.copyFrom(this);
            matrix.fill_with_identity();
            return false;
        }
        let det_inv = 1. / det;
        let det_12_33 = m12 * m33 - m32 * m13;
        let det_11_33 = m11 * m33 - m31 * m13;
        let det_11_32 = m11 * m32 - m31 * m12;
        let det_10_33 = m10 * m33 - m30 * m13;
        let det_10_32 = m10 * m32 - m30 * m12;
        let det_10_31 = m10 * m31 - m30 * m11;
        let det_12_23 = m12 * m23 - m22 * m13;
        let det_11_23 = m11 * m23 - m21 * m13;
        let det_11_22 = m11 * m22 - m21 * m12;
        let det_10_23 = m10 * m23 - m20 * m13;
        let det_10_22 = m10 * m22 - m20 * m12;
        let det_10_21 = m10 * m21 - m20 * m11;
        let cofact_10 = 0. - (m01 * det_22_33 - m02 * det_21_33 + m03 * det_21_32);
        let cofact_11 = 0. + (m00 * det_22_33 - m02 * det_20_33 + m03 * det_20_32);
        let cofact_12 = 0. - (m00 * det_21_33 - m01 * det_20_33 + m03 * det_20_31);
        let cofact_13 = 0. + (m00 * det_21_32 - m01 * det_20_32 + m02 * det_20_31);
        let cofact_20 = 0. + (m01 * det_12_33 - m02 * det_11_33 + m03 * det_11_32);
        let cofact_21 = 0. - (m00 * det_12_33 - m02 * det_10_33 + m03 * det_10_32);
        let cofact_22 = 0. + (m00 * det_11_33 - m01 * det_10_33 + m03 * det_10_31);
        let cofact_23 = 0. - (m00 * det_11_32 - m01 * det_10_32 + m02 * det_10_31);
        let cofact_30 = 0. - (m01 * det_12_23 - m02 * det_11_23 + m03 * det_11_22);
        let cofact_31 = 0. + (m00 * det_12_23 - m02 * det_10_23 + m03 * det_10_22);
        let cofact_32 = 0. - (m00 * det_11_23 - m01 * det_10_23 + m03 * det_10_21);
        let cofact_33 = 0. + (m00 * det_11_22 - m01 * det_10_22 + m02 * det_10_21);

        m[ 0] = cofact_00 * det_inv;
        m[ 1] = cofact_10 * det_inv;
        m[ 2] = cofact_20 * det_inv;
        m[ 3] = cofact_30 * det_inv;
        m[ 4] = cofact_01 * det_inv;
        m[ 5] = cofact_11 * det_inv;
        m[ 6] = cofact_21 * det_inv;
        m[ 7] = cofact_31 * det_inv;
        m[ 8] = cofact_02 * det_inv;
        m[ 9] = cofact_12 * det_inv;
        m[10] = cofact_22 * det_inv;
        m[11] = cofact_32 * det_inv;
        m[12] = cofact_03 * det_inv;
        m[13] = cofact_13 * det_inv;
        m[14] = cofact_23 * det_inv;
        m[15] = cofact_33 * det_inv;
        return true;
    }
    fn mul_to(a: &Matrix, b: &Matrix, y: & mut Matrix) {
        a.mul_to(b, y);
    }
    fn matrix4_mul_vector4(a: &Matrix, b: &Vector4, y: & mut Vector4) {
        a.mul_to(b, y);
    }
    fn matrix4_mul_matrix4(a: &Matrix, b: &Matrix, y: & mut Matrix) {
        a.mul_to(b, y);
    }
    fn matrix4_compose(scaling: &Vector3, quaternion: &Quaternion, translation: &Vector3, result: &mut Matrix) {
        let rotation = quaternion.to_rotation_matrix();
        Self::matrix4_compose_rotation(scaling, &rotation, translation, result);
    }

    fn matrix4_from_xyz_axes(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3, result: &mut Matrix) {
        result.copy_from_slice(&[
            axis1.x, axis1.y, axis1.z, 0.,
            axis2.x, axis2.y, axis2.z, 0.,
            axis3.x, axis3.y, axis3.z, 0.,
                 0.,      0.,      0., 1.,
        ]);
    }

    fn matrix4_decompose(m: &Matrix, scaling: Option<&mut Vector3>, quaternion: Option<&mut Quaternion>, translation: Option<&mut Vector3>) -> bool {
        match quaternion {
            Some(quaternion) => {
                let mut rotation = Rotation3::default();
                let result = Self::matrix4_decompose_rotation(m, scaling, Some(&mut rotation), translation);

                *quaternion = Quaternion::from_rotation_matrix(&rotation);

                result
            },
            None => {
                Self::matrix4_decompose_rotation(m, scaling, None, translation)
            },
        }
    }

    fn matrix4_compose_euler_angle(scaling: &Vector3, eulers: &Vector3, translation: &Vector3, result: &mut Matrix) {
        let rotation = Rotation3::from_euler_angles(eulers.z, eulers.x, eulers.y);
        Self::matrix4_compose_rotation(scaling, &rotation, translation, result);
    }

    fn matrix4_compose_rotation(scaling: &Vector3, rotmat: &Rotation3, translation: &Vector3, result: &mut Matrix) {
        result.fill_with_identity();

        result.fixed_view_mut::<3, 3>(0, 0).copy_from(rotmat.matrix());
        // result.append_nonuniform_scaling_mut(scaling);
        result.prepend_nonuniform_scaling_mut(scaling);
    
        result.append_translation_mut(translation);
        // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
    }
    fn matrix4_compose_no_rotation(scaling: &Vector3, translation: &Vector3, result: &mut Matrix) {
        result.fill_with_identity();
        result.append_nonuniform_scaling_mut(scaling);
        result.append_translation_mut(translation);
        // CoordinateSytem3::matrix4_compose_rotation(scaling, rotmat, translation, result)
    }

    fn matrix4_decompose_rotation(m: &Matrix, scaling: Option<&mut Vector3>, rotation: Option<&mut Rotation3>, translation: Option<&mut Vector3>) -> bool {
        // todo!()
        if let Some(translation) = translation {
            translation.copy_from(&m.fixed_view::<3, 1>(0, 3));
        }

        if m.is_identity(Number::EPSILON) {
            if let Some(scaling) =  scaling {
                scaling.x = 1.; scaling.y = 1.; scaling.z = 1.; 
            }
            if let Some(rotation) =  rotation {
                *rotation = Rotation3::identity();
                // *rotation = Self::rotation_matrix_from_euler_angles(0., 0., 0.);
            }

            return true;
        }
        else {
            let det = m.determinant();

            let m00 = m[(0, 0)];
            let m01 = m[(0, 1)];
            let m02 = m[(0, 2)];
            let m10 = m[(1, 0)];
            let m11 = m[(1, 1)];
            let m12 = m[(1, 2)];
            let m20 = m[(2, 0)];
            let m21 = m[(2, 1)];
            let m22 = m[(2, 2)];

            let sx = Number::sqrt(m00 * m00 + m01 * m01 + m02 * m02);
            let mut sy = Number::sqrt(m10 * m10 + m11 * m11 + m12 * m12);
            let sz = Number::sqrt(m20 * m20 + m21 * m21 + m22 * m22);

            if det < 0. {
                sy *= -1.;
            }
            
            if let Some(scaling) = scaling {
                scaling.x = sx; scaling.y = sy; scaling.z = sz;
            }

            if Number::EPSILON.eq(&sx.abs()) || Number::EPSILON.eq(&sy.abs()) || Number::EPSILON.eq(&sz.abs()) {
                if let Some(rotation) =  rotation {
                    rotation.clone_from(&Rotation3::from_euler_angles(0., 0., 0.));
                }
                return false;
            } else {
                if let Some(rotation) =  rotation {
                    rotation.matrix_mut_unchecked().copy_from_slice(&[
                        m00 / sx, m10 / sy, m20 / sz,
                        m01 / sx, m11 / sy, m21 / sz,
                        m02 / sx, m12 / sy, m22 / sz,
                    ]);
                }
                return true;
            }
        }
    }

    fn matrix4_compose_quaternion(scale: &Vector3, quaternion: &Quaternion, translation: &Vector3, result: &mut Matrix) {
        let m = result.as_mut_slice();
        let x = quaternion.i; let y = quaternion.j; let z = quaternion.k; let w = quaternion.w;
        let x2 = x + x; let y2 = y + y; let z2 = z + z;
        let xx = x * x2; let xy = x * y2; let xz = x * z2;
        let yy = y * y2; let yz = y * z2; let zz = z * z2;
        let wx = w * x2; let wy = w * y2; let wz = w * z2;
        let sx = scale.x; let sy = scale.y; let sz = scale.z;
        m[ 0] = (1. - (yy + zz)) * sx;
        m[ 1] = (xy + wz) * sx;
        m[ 2] = (xz - wy) * sx;
        m[ 3] = 0.;
        m[ 4] = (xy - wz) * sy;
        m[ 5] = (1. - (xx + zz)) * sy;
        m[ 6] = (yz + wx) * sy;
        m[ 7] = 0.;
        m[ 8] = (xz + wy) * sz;
        m[ 9] = (yz - wx) * sz;
        m[10] = (1. - (xx + yy)) * sz;
        m[11] = 0.;
        m[12] = translation.x;
        m[13] = translation.y;
        m[14] = translation.z;
        m[15] = 1.;
    }

    fn rotation_align_to(from: &Vector3, to: &Vector3, result: &mut Matrix) {
        let v: Vector3 = to.cross(from);
        let c = to.dot(from);
        let k = 1. / (1. + c);

        result.fill_with_identity();

        let m_00: Number = v.x * v.x * k + c;       let m_01: Number = v.y * v.x * k - v.z;     let m_02: Number = v.z * v.x * k + v.y;     let m_03: Number = 0.;
        let m_04: Number = v.x * v.y * k + v.z;     let m_05: Number = v.y * v.y * k + c;       let m_06: Number = v.z * v.y * k - v.x;     let m_07: Number = 0.;
        let m_08: Number = v.x * v.z * k - v.y;     let m_09: Number = v.y * v.z * k + v.x;     let m_10: Number = v.z * v.z * k + c;       let m_11: Number = 0.;
        let m_12: Number = 0.;                      let m_13: Number = 0.;                      let m_14: Number = 0.;                      let m_15: Number = 1.;

        result.set_column(0, &Vector4::new(m_00, m_04, m_08, m_12));
        result.set_column(1, &Vector4::new(m_01, m_05, m_09, m_13));
        result.set_column(2, &Vector4::new(m_02, m_06, m_10, m_14));
        result.set_column(3, &Vector4::new(m_03, m_07, m_11, m_15));
    }

    fn lookat(&self, eye: &Vector3, target: &Vector3, up: &Vector3, result: &mut Isometry3) {
        let eye = Point3::from_slice(eye.as_slice());
        let target = Point3::from_slice(target.as_slice());

        match self.mode {
            ECoordinateSytem3::Left => { *result = Isometry3::look_at_lh(&eye, &target, up); },
            ECoordinateSytem3::Right => { *result = Isometry3::look_at_rh(&eye, &target, up); },
        }
        
    }
}

impl TToolRotation for CoordinateSytem3 {
    fn quaternion_from_euler_angles(x: Number, y: Number, z: Number) -> Quaternion {
        Quaternion::from_rotation_matrix(&Self::rotation_matrix_from_euler_angles(x, y, z))
    }

    fn quaternion_mut_yaw_pitch_roll(&self, yaw: Number, pitch: Number, roll: Number, result: &mut Quaternion) {
        *result = Quaternion::from_rotation_matrix(&Self::rotation_matrix_from_euler_angles(yaw, pitch, roll));
    }

    fn quaternion_from_unit_vector(axis: &nalgebra::Unit<Vector3>, vec_to: &Vector3) -> Quaternion {
        let r = Vector3::dot(axis, vec_to) + 1.0;
        let quat = if r < f32::EPSILON {
            if f32::abs(axis.x) > f32::abs(axis.z) {
                nalgebra::Quaternion::new(0., -1.0 * axis.y, axis.x, 0.)
            } else {
                nalgebra::Quaternion::new(0., 0.0, -1.0 * axis.z, axis.y)
            }
        } else {
            let temp = Vector3::cross(axis, vec_to);
            nalgebra::Quaternion::new(r, temp.x, temp.y, temp.z)
        };
        Quaternion::from_quaternion(quat)
    }

    fn quaternion_mut_axis(&self, _axis1: &Vector3, _axis2: &Vector3, _axis3: &Vector3, _result: &mut Quaternion) {
        todo!()
    }

    fn quaternion_to_euler_angles(&self, quaternion: &Quaternion, result: &mut Vector3) {
        let (z, x, y) = quaternion.euler_angles();
        // match self.mode {
        //     ECoordinateSytem3::Left => {
        //         result.copy_from_slice(&[-x, -y, -z]);
        //     },
        //     ECoordinateSytem3::Right => {
        //         result.copy_from_slice(&[x, y, z]);
        //     },
        // }
        result.copy_from_slice(&[x, y, z]);
    }

    fn rotation_matrix_from_euler_angles_toref(x: Number, y: Number, z: Number, result: &mut Rotation3) {
        let (sr, cr) = x.simd_sin_cos();
        let (sp, cp) = y.simd_sin_cos();
        let (sy, cy) = z.simd_sin_cos();

        result.matrix_mut_unchecked().copy_from_slice(&[
            cy * cp,
            cy * sp * sr - sy * cr,
            cy * sp * cr + sy * sr,
            sy * cp,
            sy * sp * sr + cy * cr,
            sy * sp* cr - cy * sr,
            -sp,
            cp * sr,
            cp * cr,
        ]);
    }
    fn rotation_matrix_from_euler_angles(x: Number, y: Number, z: Number) -> Rotation3 {
        // match self.mode {
        //     ECoordinateSytem3::Left => {
        //         Rotation3::from_euler_angles(-x, -y, -z)
        //     },
        //     ECoordinateSytem3::Right => {
        //         Rotation3::from_euler_angles(x, y, z)
        //     },
        // }

        // Rotation3::from_euler_angles(x, y, z)
        
        let (sr, cr) = x.simd_sin_cos();
        let (sp, cp) = y.simd_sin_cos();
        let (sy, cy) = z.simd_sin_cos();

        Rotation3::from_matrix_unchecked(Matrix3::from_column_slice(
            &[
                cy * cp,
                cy * sp * sr - sy * cr,
                cy * sp * cr + sy * sr,
                sy * cp,
                sy * sp * sr + cy * cr,
                sy * sp* cr - cy * sr,
                -sp,
                cp * sr,
                cp * cr,
            ]
        ))
    }

    fn rotation_matrix_mut_yaw_pitch_roll(&self, yaw: Number, pitch: Number, roll: Number, result: &mut Rotation3) {
        // match self.mode {
        //     ECoordinateSytem3::Left => {
        //         result.clone_from(&Rotation3::from_euler_angles(-roll, -pitch, -yaw));
        //     },
        //     ECoordinateSytem3::Right => {
        //         result.clone_from(&Rotation3::from_euler_angles(roll, pitch, yaw));
        //     },
        // }
        *result = Rotation3::from_euler_angles(roll, pitch, yaw);
    }

    fn rotation_matrix_mut_axis(&self, _axis1: &Vector3, _axis2: &Vector3, _axis3: &Vector3, _result: &mut Rotation3) {
        todo!()
    }

    fn rotation_matrix_to_euler_angles(&self, rotation: &Rotation3, result: &mut Vector3) {
        let (z, x, y) = rotation.euler_angles();
        // match self.mode {
        //     ECoordinateSytem3::Left => {
        //         result.copy_from_slice(&[-x, -y, -z]);
        //     },
        //     ECoordinateSytem3::Right => {
        //         result.copy_from_slice(&[x, y, z]);
        //     },
        // }
        result.copy_from_slice(&[x, y, z]);
    }

    fn quaternion_mut_euler_angles(x: Number, y: Number, z: Number, result: &mut Quaternion) {
        *result = Self::quaternion_from_euler_angles(x, y, z);
    }

    fn rotation_matrix_mut_euler_angles(x: Number, y: Number, z: Number, result: &mut Rotation3) {
        *result = Self::rotation_matrix_from_euler_angles(x, y, z);
    }
    ///
    /// 会卡死原因未知
    fn rotation_matrix_from_axises(axis1: &Vector3, axis2: &Vector3, axis3: &Vector3) -> Rotation3 {
        let mut m = Matrix3::identity();
        m.copy_from_slice(&[
            axis1.x, axis1.y, axis1.z,
            axis2.x, axis2.y, axis2.z,
            axis3.x, axis3.y, axis3.z,
        ]);
        return Rotation3::from_matrix(&m);
    }
    fn quaternion_from_axis_angle(axis1: &Vector3, radian: Number) -> Quaternion {
        let (sin, cos) = f32::simd_sin_cos(radian / 2.);
        let quat = nalgebra::Quaternion::new(
            cos,
            axis1.x * sin,
            axis1.y * sin,
            axis1.z * sin,
        );
        return Quaternion::from_quaternion(quat);
    }

}