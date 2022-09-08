use pi_scene_math::{vector::{ TMinimizeMaximize }, Vector3, Matrix, frustum::FrustumPlanes};


pub struct BoundingSphere {
    radius: f32,
    center: Vector3,
    radius_world: f32,
    center_world: Vector3,
}

impl Default for BoundingSphere {
    fn default() -> Self {
        Self { radius: 1., center: Vector3::zeros(), radius_world: 1., center_world: Vector3::zeros() }
    }
}

impl BoundingSphere {
    pub fn new(min: &Vector3, max: &Vector3, world: &Matrix) -> Self {
        let mut result = BoundingSphere::default();

        result.reset(min, max, world);

        result
    }

    pub fn reset(&mut self, min: &Vector3, max: &Vector3, world: &Matrix) {

        let distance = min.metric_distance(max);

        max.add_to(min, &mut self.center);
        self.center.scale_mut(0.5);
        self.radius *= 0.5;

        self.update(world);
    }

    pub fn update(&mut self, world: &Matrix) {
        if world.is_identity(0.00001) {
            self.radius = self.radius;
            self.center_world.copy_from(&self.center);
        } else {
            // 计算 center_world
            // 计算 radius_world
            // world.
        }
    }

    pub fn is_center_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        // TODo
        true
    }

    pub fn is_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        true
    }
}