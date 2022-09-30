use render_data_container::{TVertexBufferKindKey, TMaterialBlockKindKey, TGeometryBufferID, TextureID};
use pi_scene_math::Matrix;
use render_geometry::geometry::Geometry;
use render_material::material::Material;


pub struct Mesh<VBK: TVertexBufferKindKey, MBKK: TMaterialBlockKindKey, GBID: TGeometryBufferID, TID: TextureID> {
    pub geometry: Vec<Geometry<VBK, GBID>>,
    pub material: Vec<Material<VBK, MBKK, TID>>,
    pub render_matrix: Matrix,
}