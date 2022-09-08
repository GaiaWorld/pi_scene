use pi_scene_math::{vector::{ TMinimizeMaximize }, Vector3, Matrix, plane::Plane, frustum::FrustumPlanes};

use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere};

pub mod bounding_box;
pub mod bounding_sphere;

pub trait TIntersect {
    fn intersects_point(&self, p: &Vector3) -> bool;
    fn intersects_box(&self, b: &BoundingBox) -> bool;
    fn intersects_sphere(&self, s: &BoundingSphere) -> bool;
    fn intersects_min_max(&self, min: &Vector3, max: &Vector3) -> bool;
}

pub struct BoundingInfo {
    minimum: Vector3,
    maximum: Vector3,
    _box: BoundingBox,
    _sphere: BoundingSphere,
    direction0: Vector3,
    direction1: Vector3,
    direction2: Vector3,
}

impl Default for BoundingInfo {
    fn default() -> Self {
        
        let minimum: Vector3 = Vector3::new(-1., -1., -1.);
        let maximum: Vector3 = Vector3::new(1., 1., 1.);
        let world: Matrix = Matrix::identity();

        let _box = BoundingBox::new(&minimum, &maximum, &world);
        let _sphere = BoundingSphere::new(&minimum, &maximum, &world);
        Self {
            minimum, maximum, _box, _sphere,
            direction0: Vector3::zeros(), direction1: Vector3::zeros(), direction2: Vector3::zeros(),
        }
    }
}

impl BoundingInfo {
    pub fn reset(&mut self, min: &Vector3, max: &Vector3, world: &Matrix) {
        self.minimum.copy_from(min);
        self.maximum.copy_from(max);
        self._box.reset(min, max, world);
        self._sphere.reset(min, max, world);

        self.direction0.copy_from(&world.slice((0, 0), (1, 3)));
        self.direction1.copy_from(&world.slice((1, 0), (1, 3)));
        self.direction2.copy_from(&world.slice((2, 0), (1, 3)));
    }

    pub fn update(&mut self, world: &Matrix) {
        self._box.reset(&self.minimum, &self.maximum, world);
        self._sphere.update(world);
    }
}

pub fn check_boundings(boundings: &Vec<BoundingInfo>, frustum_planes: &FrustumPlanes, result: &mut Vec<bool>) {

}