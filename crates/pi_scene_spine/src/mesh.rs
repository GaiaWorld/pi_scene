use pi_scene_geometry::{geometry::{Geometry, GeometryDataDesc}, TVertexDataKindKey, vertex_data::EVertexDataFormat};

use crate::MAX_VERTICES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EMeshKind {
    Vertices,
    Indices,
}

impl TVertexDataKindKey for EMeshKind {}

pub struct Mesh {
    geometry_desc: GeometryDataDesc<EMeshKind>,
    geometry: Geometry<EMeshKind>,
    attributes: Vec<VertexAttribute>,
    num_vertices: u32,
    num_indices: u32,
    dirty_vertices: bool,
    used_vertices: u32,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            geometry_desc: GeometryDataDesc(EMeshKind::Vertices, EVertexDataFormat::F32),
            geometry: Geometry::new(),
            attributes: vec![],
            num_vertices: MAX_VERTICES as u32,
            num_indices: 0,
            dirty_vertices: false,
            used_vertices: 0,
        }
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn element_per_vertex(&self) -> u32 {
        let mut result = 0;
        for v in self.attributes.iter() {
            result += v.num_elements;
        }

        result
    }
    pub fn get_attributes(&self) -> &Vec<VertexAttribute> {
        &self.attributes
    }
    pub fn max_vertices(&self) -> u32 {
        MAX_VERTICES as u32
    }
    pub fn num_vertices(&self) -> u32 {
        self.geometry.get(self.geometry_desc).f32.unwrap().size
    }
    pub fn set_vertices_length(&mut self, length: u32) {
        self.dirty_vertices = true;
        self.used_vertices = length;
    }
    // pub fn get_vertices(&mut self) -> &mut  {

    // }
}


pub struct VertexAttribute {
    name: String,
    ty: EVertexDataFormat,
    num_elements: u32,
}
