use crate::{Vector3, Number};


pub struct Plane {
    pub normal: Vector3,
    pub d: Number,
}

impl Plane {
    pub fn from_points(&mut self, p1: &Vector3, p2: &Vector3, p3: &Vector3) {

    }
    pub fn from_point_and_normal(&mut self, origin: &Vector3, normal: &Vector3) {

    }
}