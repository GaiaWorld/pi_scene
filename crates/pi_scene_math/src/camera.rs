
use crate::{Matrix, Number, coordiante_system::CoordinateSytem3, Perspective3, Orthographic3};

pub trait TOrthographicCameraTool {
    fn orthographic_rh(
        left: Number,
        right: Number,
        bottom: Number,
        top: Number,
        znear: Number,
        zfar: Number,
    ) -> Matrix;
    fn orthographic_lh(
        left: Number,
        right: Number,
        bottom: Number,
        top: Number,
        znear: Number,
        zfar: Number,
    ) -> Matrix;
}

impl TOrthographicCameraTool for CoordinateSytem3 {
    fn orthographic_rh(
        left: Number,
        right: Number,
        bottom: Number,
        top: Number,
        znear: Number,
        zfar: Number,
    ) -> Matrix {
        let mut result = Self::orthographic_lh(left, right, bottom, top, znear, zfar);
        result[(2, 2)] = -result[(2, 2)];

        return result;
    }

    fn orthographic_lh(
        left: Number,
        right: Number,
        bottom: Number,
        top: Number,
        znear: Number,
        zfar: Number,
    ) -> Matrix {
        let half_z_range: Matrix = Matrix::from_column_slice([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.].as_slice());
        let n = znear;
        let f = zfar;

        let a = 2.0 / (right - left);
        let b = 2.0 / (top - bottom);
        let c = 2.0 / (f - n);
        let d = -(f + n) / (f - n);
        let i0 = (left + right) / (left - right);
        let i1 = (top + bottom) / (bottom - top);

        let result = Matrix::from_column_slice(&[
            a,      0.0,    0.0,    0.0,
            0.0,    b,      0.0,    0.0,
            0.0,    0.0,    c,      0.0,
            i0,     i1,     d,      1.0,
        ]);

        // println!(">>>>>>>>>> P: {:?}", result);
        // println!(">>>>>>>>>> HP: {:?}", half_z_range * result);

        return half_z_range * result;
        
        // Orthographic3::new(left, right, bottom, top, znear, zfar).as_matrix() * half_z_range
        // if is_vertical_fixed {
        //     Orthographic3::new(left, right, bottom, top, znear, zfar).as_matrix().clone()
        // } else {
        //     Perspective3::new(aspect, fov / aspect, znear, zfar).as_matrix().clone()
        // }
    }
}

pub trait TPerspectiveCameraTool {
    fn perspective_for_reverse_rh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix;
    fn perspective_rh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix;
    fn perspective_for_reverse_lh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix;
    fn perspective_lh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix;
}

impl TPerspectiveCameraTool for CoordinateSytem3 {
    fn perspective_for_reverse_rh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix {
        let half_z_range: Matrix = Matrix::from_column_slice([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.].as_slice());
        let n = znear;
        let f = zfar;

        let t = 1.0 / (Number::tan(fov * 0.5));
        let a = if is_vertical_fixed { t / aspect } else { t };
        let b = if is_vertical_fixed { t } else { t * aspect };
        let _c = (f + n) / (f - n);
        let _d = -2.0 * f * n / (f - n);

        let result = Matrix::from_column_slice(&[
            a, 0.0, 0.0, 0.0,
            0.0, b, 0.0, 0.0,
            0.0, 0.0, -znear, -1.0,
            0.0, 0.0, -1.0, 0.0,
        ]);

        return half_z_range * result;

        // if is_vertical_fixed {
        //     Perspective3::new(aspect, fov, znear, zfar).as_matrix() * half_z_range
        // } else {
        //     Perspective3::new(aspect, fov / aspect, znear, zfar).as_matrix() * half_z_range
        // }
    }

    fn perspective_rh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix {
        let half_z_range: Matrix = Matrix::from_column_slice([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.].as_slice());
        let n = znear;
        let f = zfar;

        let t = 1.0 / (Number::tan(fov * 0.5));
        let a = if is_vertical_fixed { t / aspect } else { t };
        let b = if is_vertical_fixed { t } else { t * aspect };
        let c = -(f + n) / (f - n);
        let d = -2.0 * f * n / (f - n);
        
        let result = Matrix::from_column_slice(&[
            a, 0.0, 0.0, 0.0,
            0.0, b, 0.0, 0.0,
            0.0, 0.0, c, -1.0,
            0.0, 0.0, d, 0.0,
        ]);

        return half_z_range * result;
        

        // if is_vertical_fixed {
        //     Perspective3::new(aspect, fov, znear, zfar).as_matrix() * half_z_range
        // } else {
        //     Perspective3::new(aspect, fov / aspect, znear, zfar).as_matrix() * half_z_range
        // }
    }

    fn perspective_for_reverse_lh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix {
        let half_z_range: Matrix = Matrix::from_column_slice([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.].as_slice());
        let n = znear;
        let f = zfar;

        let t = 1.0 / (Number::tan(fov * 0.5));
        let a = if is_vertical_fixed { t / aspect } else { t };
        let b = if is_vertical_fixed { t } else { t * aspect };
        let _c = (f + n) / (f - n);
        let _d = -2.0 * f * n / (f - n);

        let result = Matrix::from_column_slice(&[
            a, 0.0, 0.0, 0.0,
            0.0, b, 0.0, 0.0,
            0.0, 0.0, -znear, 1.0,
            0.0, 0.0, -1.0, 0.0,
        ]);

        return half_z_range * result;

        // if is_vertical_fixed {
        //     Perspective3::new(aspect, fov, znear, zfar).as_matrix() * half_z_range
        // } else {
        //     Perspective3::new(aspect, fov / aspect, znear, zfar).as_matrix() * half_z_range
        // }
    }

    fn perspective_lh(
        fov: Number,
        aspect: Number,
        znear: Number,
        zfar: Number,
        is_vertical_fixed: bool
    ) -> Matrix {
        let half_z_range: Matrix = Matrix::from_column_slice([1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.5, 0., 0., 0., 0.5, 1.].as_slice());
        let n = znear;
        let f = zfar;

        let t = 1.0 / (Number::tan(fov * 0.5));
        let a = if is_vertical_fixed { t / aspect } else { t };
        let b = if is_vertical_fixed { t } else { t * aspect };
        let c = (f + n) / (f - n);
        let d = -2.0 * f * n / (f - n);

        let result = Matrix::from_column_slice(&[
            a, 0.0, 0.0, 0.0,
            0.0, b, 0.0, 0.0,
            0.0, 0.0, c, 1.0,
            0.0, 0.0, d, 0.0,
        ]);

        return half_z_range * result;
        
        // if is_vertical_fixed {
        //     Perspective3::new(aspect, fov, znear, zfar).as_matrix() * half_z_range
        // } else {
        //     Perspective3::new(aspect, fov / aspect, znear, zfar).as_matrix() * half_z_range
        // }
    }
}