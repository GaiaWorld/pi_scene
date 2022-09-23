
use pi_scene_geometry::{geometry::Geometry, };
use pi_scene_data_container::{TMaterialBlockKindKey, TextureID, TVertexBufferKindKey, TGeometryBufferID, };
use pi_scene_material::material::Material;
use pi_scene_math::Matrix;


pub struct Mesh<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, GBID: TGeometryBufferID, TID: TextureID> {
    pub geometry: Vec<Geometry<VBK, GBID>>,
    pub material: Vec<Material<VBK, MBKK, TID>>,
    pub render_matrix: Matrix,
}