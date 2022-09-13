use pi_scene_math::{Color3, Matrix};

use crate::transform_node::TransformNode;

use super::direction_light::DirectionLight;



pub struct ShadowParam {
    /// 偏移矫正
    pub bias: f32,
    /// 法线方向偏移矫正
    pub normal_bias: f32,
    /// 阴影强度
    pub strength: f32,
    /// 阴影颜色
    pub color: Color3,
}

pub struct DirectionShadowCaster {
    /// 光照方向截面尺寸
    pub view_size: f32,
    /// 光照方向容纳深度
    pub view_depth: f32,
    /// 使用的纹理目标尺寸
    pub atlas_size: u32,
    pub view_matrix: Matrix,
    pub project_matrix: Matrix,
}

pub fn calc_camera_shadow_direction(light: &DirectionLight, node: &TransformNode, caster: &mut DirectionShadowCaster) {

}