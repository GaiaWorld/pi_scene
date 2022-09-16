use pi_scene_geometry::TVertexDataKindKey;
use pi_scene_material::{material::TMaterialBlockKindKey, texture::TextureKey};

use crate::meshes::mesh::Mesh;


pub fn scene0_camera0_meshes<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey>(meshes: Vec<Mesh<K, K0, K2D>>) {

}
/// 实例化 mesh
pub fn scene0_camera0_meshes_instanced<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey>(meshes: Vec<Mesh<K, K0, K2D>>) {
    
}
/// 合并 mesh
pub fn scene0_camera0_meshes_static<K: TVertexDataKindKey, K0: TMaterialBlockKindKey, K2D: TextureKey>(meshes: Vec<Mesh<K, K0, K2D>>) {
    
}