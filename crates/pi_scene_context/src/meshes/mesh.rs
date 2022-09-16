use std::sync::Arc;

use pi_scene_geometry::{geometry::Geometry, TVertexDataKindKey};
use pi_scene_material::{material::{Material, TMaterialBlockKindKey}, texture::TextureKey};
use pi_scene_math::Matrix;


pub struct Mesh<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey> {
    pub geometry: Vec<Geometry<K>>,
    pub material: Vec<Material<K, K0, K2D>>,
    pub render_matrix: Matrix,
}