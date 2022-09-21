use parry3d::shape::ConvexPolyhedron;
use pi_scene_math::{
    frustum::FrustumPlanes, plane::Plane, vector::TMinimizeMaximize, Matrix, Vector3,
};
use pi_slotmap::{DefaultKey, KeyData, Key};
use std::hash;
use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere};

pub mod bounding_box;
pub mod bounding_sphere;
pub mod oct_tree;
/// 检测级别
/// *
pub enum ECullingStrategy {
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥
    Optimistic,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥, 检测 包围盒 在不在 视锥
    STANDARD,
}

pub trait TIntersect {
    fn intersects_point(&self, p: &Vector3) -> bool;
    fn intersects_box(&self, b: &BoundingBox) -> bool;
    fn intersects_sphere(&self, s: &BoundingSphere) -> bool;
    fn intersects_min_max(&self, min: &Vector3, max: &Vector3) -> bool;
}

pub struct BoundingInfo {
    minimum: Vector3,
    maximum: Vector3,
    pub bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    direction0: Vector3,
    direction1: Vector3,
    direction2: Vector3,
    pub culling_strategy: ECullingStrategy,
}

impl Default for BoundingInfo {
    fn default() -> Self {
        let minimum: Vector3 = Vector3::new(-1., -1., -1.);
        let maximum: Vector3 = Vector3::new(1., 1., 1.);
        let world: Matrix = Matrix::identity();

        let bounding_box = BoundingBox::new(&minimum, &maximum, &world);
        let bounding_sphere = BoundingSphere::new(&minimum, &maximum, &world);
        Self {
            minimum,
            maximum,
            bounding_box,
            bounding_sphere,
            direction0: Vector3::zeros(),
            direction1: Vector3::zeros(),
            direction2: Vector3::zeros(),
            culling_strategy: ECullingStrategy::STANDARD,
        }
    }
}

impl BoundingInfo {
    pub fn reset(&mut self, min: &Vector3, max: &Vector3, world: &Matrix) {
        self.minimum.copy_from(min);
        self.maximum.copy_from(max);
        self.bounding_box.reset(min, max, world);
        self.bounding_sphere.reset(min, max, world);

        self.direction0.copy_from(&world.slice((0, 0), (1, 3)));
        self.direction1.copy_from(&world.slice((1, 0), (1, 3)));
        self.direction2.copy_from(&world.slice((2, 0), (1, 3)));
    }

    pub fn update(&mut self, world: &Matrix) {
        self.bounding_box.reset(&self.minimum, &self.maximum, world);
        self.bounding_sphere.update(world);
    }

    pub fn is_in_frustum(&self, frustum_planes: &FrustumPlanes) -> bool {
        // TODO; 是否需要加上这句
        // if self.bounding_sphere.is_center_in_frustum(frustum_planes) {
        //     return true;
        // }

        if !self.bounding_sphere.is_in_frustum(frustum_planes) {
            return false;
        }

        return self.bounding_box.is_in_frustum(frustum_planes);
    }
}

pub fn check_boundings(
    boundings: &Vec<BoundingInfo>,
    frustum_planes: &FrustumPlanes,
    result: &mut Vec<bool>,
) {
    let len = boundings.len();
    let mut res_vec = Vec::with_capacity(len);
    for index in 0..len {
        let is_in_frustum = boundings[index].is_in_frustum(frustum_planes);
        res_vec.push(is_in_frustum);
    }
    *result = res_vec;
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct BoundingKey(pub(crate) usize);

unsafe impl Key for BoundingKey {
    #[inline]
    fn data(&self) -> KeyData {
        KeyData::from_ffi(self.0 as u64)
    }
}

impl From<KeyData> for BoundingKey {
    #[inline]
    fn from(data: KeyData) -> Self {
        BoundingKey(data.as_ffi() as usize)
    }
}

pub trait TBoundingInfoCalc {
    fn add(&mut self, key: BoundingKey, info: BoundingInfo);
    fn remove(&mut self, key: BoundingKey);
    fn check_boundings(
        &self,
        frustum_planes: &FrustumPlanes,
        result: &mut Vec<BoundingKey>
    );
    fn check_boundings_of_tree(
        &self,
        frustum_planes: &ConvexPolyhedron,
    );
}

pub struct VecBoundingInfoCalc {
    recycle: Vec<usize>,
    record: Vec<usize>,
    list: Vec<BoundingInfo>,
}

impl Default for VecBoundingInfoCalc {
    fn default() -> Self {
        Self {
            recycle: vec![],
            record: vec![],
            list: vec![],
        }
    }
}

impl TBoundingInfoCalc for VecBoundingInfoCalc {
    fn add(&mut self, key: BoundingKey, info: BoundingInfo) {
        match self.recycle.pop() {
            Some(index) => {
                self.list[index] = info;
                self.record[index] = key.0;
            },
            None => {
                self.list.push(info);
                self.record.push(key.0);
            },
        } 
    }

    fn remove(&mut self, key: BoundingKey) {
        match self.record.binary_search(&key.0) {
            Ok(index) => {
                self.recycle.push(index);
            },
            Err(_) => {},
        }
    }

    fn check_boundings(
        &self, 
        frustum_planes: &FrustumPlanes,
        result: &mut Vec<BoundingKey>
    ) {
        let len = self.list.len();
        let mut res_vec = Vec::with_capacity(len);
        for index in 0..len {
            if !self.recycle.contains(&index) {
                if self.list[index].is_in_frustum(frustum_planes) {
                    let key = self.record.get(index).unwrap();
                    res_vec.push(BoundingKey(*key));
                }
            }
        }
        *result = res_vec;
    }
    fn check_boundings_of_tree(
        &self,
        _frustum_planes: &ConvexPolyhedron,
    ){
        todo!()
    }
}
