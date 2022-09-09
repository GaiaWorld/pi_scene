use std::sync::Arc;

use pi_scene_geometry::{geometry::Geometry, TVertexDataKindKey};
use pi_scene_material::{Material, TMaterialBlockKindKey};
use pi_scene_math::Matrix;


pub struct Mesh<K: TVertexDataKindKey, K0: TMaterialBlockKindKey> {
    pub geometry: Vec<Geometry<K>>,
    pub material: Vec<Material<K, K0>>,
    pub render_matrix: Matrix,
}