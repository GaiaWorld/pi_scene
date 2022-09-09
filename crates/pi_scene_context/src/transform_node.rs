use pi_scene_math::{Vector3, Quaternion, Matrix, Number, Rotation3, Affine3, transform::Transform3, coordiante_system::CoordinateSytem3, vector::TToolMatrix};

pub struct TransformNode {
    /// 变换
    transform: Transform3,
    /// 全局变换矩阵
    world_matrix: Matrix,
    absolute_position: Vector3,
    absolute_scaling: Vector3,
    absolute_rotation: Rotation3,
    dirty: bool,
}

impl Default for TransformNode {
    fn default() -> Self {
        Self::identity()
    }
}

impl TransformNode {
    pub fn identity() -> Self {
        Self {
            world_matrix: Matrix::identity(),
            transform: Transform3::identity(),
            absolute_position: Vector3::zeros(),
            absolute_scaling: Vector3::new(1., 1., 1.),
            absolute_rotation: Rotation3::identity(),
            dirty: false,
        }
    }

    pub fn position(&self) -> Vector3 {
        self.transform.translation()
    }

    pub fn position_mut(&mut self, rhs: &Vector3) {
        self.dirty = true;
        self.transform.translation_mut(rhs);
    }

    pub fn set_position_from_floats(&mut self, x: f32, y: f32, z: f32) {
        self.dirty = true;
        self.transform.set_translation_from_floats(x, y, z);
    }

    pub fn scaling(&self) -> Vector3 {
        self.transform.scaling()
    }

    pub fn scaling_mut(&mut self, rhs: &Vector3) {
        self.dirty = true;
        self.transform.scaling_mut(rhs);
    }

    pub fn set_scaling_from_floats(&mut self, x: f32, y: f32, z: f32) {
        self.dirty = true;
        self.transform.set_scaling_from_floats(x, y, z);
    }

    pub fn set_scaling_uniform(&mut self, s: f32) {
        self.dirty = true;
        self.transform.set_scaling_uniform(s);
    }

    pub fn rotation(&self, coord: &CoordinateSytem3) -> Vector3 {
        self.transform.get_rotation_euler_angles(coord)
    }

    pub fn rotation_matrix(&self) -> Rotation3 {
        self.transform.rotation_matrix()
    }
    pub fn rotation_matrix_mut(&mut self, rhs: &Rotation3) {
        self.dirty = true;
        self.transform.rotation_matrix_mut(rhs);
    }
    pub fn rotation_quaternion(&self) -> Quaternion {
        self.transform.rotation_quaternion()
    }
    pub fn rotation_quaternion_mut(&mut self, rhs: &Quaternion) {
        self.dirty = true;
        self.transform.rotation_quaternion_mut(rhs);
    }
    pub fn set_rotation_from_euler_angles(&mut self, x: Number, y: Number, z: Number, coord: &CoordinateSytem3) {
        self.dirty = true;
        self.transform.set_rotation_from_euler_angles(x, y, z, coord);
    }
    pub fn local_matrix(&self) -> &Matrix {
        self.transform.matrix()
    }
    pub fn calc_local_matrix(&mut self) {
        self.transform.calc_matrix();
    }
    pub fn world_matrix(&self) -> &Matrix {
        &self.world_matrix
    }
    /// 由外部 保证已准确地先计算完了祖先节点
    pub fn calc_world_matrix(&mut self, parent: Option<&Self>) {
        self.transform.calc_matrix();

        match parent {
            Some(parent) => {
                parent.world_matrix.mul_to(self.transform.matrix(), &mut self.world_matrix);
            },
            None => {
                self.world_matrix.copy_from(self.transform.matrix());
            },
        }

        CoordinateSytem3::matrix4_decompose_rotation(&self.world_matrix, Some(&mut self.absolute_scaling), Some(&mut self.absolute_rotation), Some(&mut self.absolute_position));

        self.dirty = false;
    }
    pub fn absolute_position(&self) -> Vector3 {
        self.absolute_position.clone()
    }
    pub fn absolute_scaling(&self) -> Vector3 {
        self.absolute_scaling.clone()
    }
    pub fn absolute_rotation(&self) -> Rotation3 {
        self.absolute_rotation.clone()
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use pi_scene_math::{coordiante_system::CoordinateSytem3, Vector3};

    use super::TransformNode;

    #[test]
    fn test_transform_node_tree_left() {
        let coord = CoordinateSytem3::default();

        let mut node0 = TransformNode::default();
        node0.set_position_from_floats(100.0, 100.0, -100.0);
        node0.set_scaling_from_floats(2.0, 2.0, 2.0);
        // node0.set_rotation_from_euler_angles(3.1415926 / 2., 0., 0.0, &coord);
        node0.set_rotation_from_euler_angles(0., 3.1415926 / 2., 0.0, &coord);
        // node0.set_rotation_from_euler_angles(0., 0., 3.1415926 / 2., &coord);
        node0.calc_world_matrix(None);

        let mut node1 = TransformNode::default();
        node1.set_position_from_floats(10.0, 0.0, 0.0);
        node1.calc_world_matrix(Some(&node0));


        println!("{:?}", node1.absolute_position());
        assert_relative_eq!(node1.absolute_position(), Vector3::new(100., 100.0, -80.), epsilon = 0.000001);
        println!("{:?}", node1.absolute_scaling());
        println!("{:?}", node1.absolute_rotation());
    }
    #[test]
    fn test_transform_node_tree_right() {
        let coord = CoordinateSytem3::right();

        let mut node0 = TransformNode::default();
        node0.set_position_from_floats(100.0, 100.0, -100.0);
        node0.set_scaling_from_floats(2.0, 2.0, 2.0);
        // node0.set_rotation_from_euler_angles(3.1415926 / 2., 0., 0.0, &coord);
        node0.set_rotation_from_euler_angles(0., 3.1415926 / 2., 0.0, &coord);
        // node0.set_rotation_from_euler_angles(0., 0., 3.1415926 / 2., &coord);
        node0.calc_world_matrix(None);

        let mut node1 = TransformNode::default();
        node1.set_position_from_floats(10.0, 0.0, 0.0);
        node1.calc_world_matrix(Some(&node0));


        println!("{:?}", node1.absolute_position());
        assert_relative_eq!(node1.absolute_position(), Vector3::new(100., 100.0, -120.), epsilon = 0.000001);
        println!("{:?}", node1.absolute_scaling());
        println!("{:?}", node1.absolute_rotation());
    }
}