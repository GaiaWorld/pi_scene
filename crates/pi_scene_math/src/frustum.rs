use crate::{plane::Plane, Matrix};

pub struct FrustumPlanes {
    pub near: Plane,
    pub far: Plane,
    pub left: Plane,
    pub right: Plane,
    pub top: Plane,
    pub bottom: Plane,
}

impl Default for FrustumPlanes {
    fn default() -> Self {
        Self { near: Plane::default(), far: Plane::default(), left: Plane::default(), right: Plane::default(), top: Plane::default(), bottom: Plane::default() }
    }
}

impl FrustumPlanes {
    /// 使用的 BABYLONJS 代码 行主序
    pub fn from_transform_matrix(&mut self, transform: &Matrix) {
        // Near
        self.transform_near_plane(transform);

        // Far
        self.transform_far_plane(transform);

        // Left
        self.transform_left_plane(transform);

        // Right
        self.transform_right_plane(transform);

        // Top
        self.transform_top_plane(transform);

        // Bottom
        self.transform_bottom_plane(transform);
    }

    pub fn transform_near_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.near.normal.x  = m[3]  + m[2];
        self.near.normal.y  = m[7]  + m[6];
        self.near.normal.z  = m[11] + m[10];
        self.near.d         = m[15] + m[14];
        self.near.normalize();
    }

    pub fn transform_far_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.far.normal.x = m[3] - m[2];
        self.far.normal.y = m[7] - m[6];
        self.far.normal.z = m[11] - m[10];
        self.far.d = m[15] - m[14];
        self.far.normalize();
    }

    pub fn transform_left_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.left.normal.x = m[3] + m[0];
        self.left.normal.y = m[7] + m[4];
        self.left.normal.z = m[11] + m[8];
        self.left.d = m[15] + m[12];
        self.left.normalize();
    }

    pub fn transform_right_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.right.normal.x = m[3] - m[0];
        self.right.normal.y = m[7] - m[4];
        self.right.normal.z = m[11] - m[8];
        self.right.d = m[15] - m[12];
        self.right.normalize();
    }

    pub fn transform_top_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.top.normal.x = m[3] - m[1];
        self.top.normal.y = m[7] - m[5];
        self.top.normal.z = m[11] - m[9];
        self.top.d = m[15] - m[13];
        self.top.normalize();
    }

    pub fn transform_bottom_plane(&mut self, transform: &Matrix) {
        let m = transform;
        self.bottom.normal.x = m[3] + m[1];
        self.bottom.normal.y = m[7] + m[5];
        self.bottom.normal.z = m[11] + m[9];
        self.bottom.d = m[15] + m[13];
        self.bottom.normalize();
    }
}
