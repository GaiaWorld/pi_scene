use crate::{Number, Vector3};

#[derive(Debug)]
pub struct Plane {
    pub normal: Vector3,
    pub d: Number,
}

impl Default for Plane {
    fn default() -> Self {
        Self { normal: Vector3::new(0., 1., 0.), d: 1. }
    }
}

impl Plane {
    pub fn from_points(&mut self, _p1: &Vector3, _p2: &Vector3, _p3: &Vector3) {}
    pub fn from_point_and_normal(&mut self, _origin: &Vector3, _normal: &Vector3) {}

    pub fn normalize(&mut self) {
        let norm = ((self.normal[0] * self.normal[0])
            + (self.normal[1] * self.normal[1])
            + (self.normal[2] * self.normal[2]))
            .sqrt();
        let mut magnitude = 0.0;

        if norm > 0.0 {
            magnitude = 1.0 / norm;
        }
        self.normal[0] *= magnitude;
        self.normal[1] *= magnitude;
        self.normal[2] *= magnitude;
        self.d *= magnitude;
    }

    pub fn dot_coordinate(&self, point: &Vector3) -> Number {
        return (((self.normal[0] * point[0]) + (self.normal[1] * point[1]))
            + (self.normal[2] * point[2]))
            + self.d;
    }
}
