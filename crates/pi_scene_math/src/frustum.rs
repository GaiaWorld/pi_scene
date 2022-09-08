use crate::{plane::Plane, Matrix};


pub struct FrustumPlanes {
    pub near: Plane,
    pub far: Plane,
    pub left: Plane,
    pub right: Plane,
    pub top: Plane,
    pub bottom: Plane,
}

impl FrustumPlanes {
    pub fn from_transform_matrix(&mut self, transform: &Matrix) {

    }
}