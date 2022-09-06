use std::sync::Arc;

use pi_scene_geometry::Geometry;
use pi_scene_material::Material;
use pi_scene_math::Matrix;


pub struct Mesh {
    pub geometry: Vec<Geometry>,
    pub material: Vec<Material>,
    pub render_matrix: Matrix,
}