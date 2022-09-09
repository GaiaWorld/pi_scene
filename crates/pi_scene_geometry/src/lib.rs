
use std::hash::Hash;

pub mod vertex_data;
pub mod error;
pub mod geometry;

pub trait TVertexDataKindKey: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {}
