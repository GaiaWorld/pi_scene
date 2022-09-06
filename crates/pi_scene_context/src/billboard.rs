///! 节点对齐模式 - 影响节点 渲染矩阵

pub enum EBillboardMod {
    None,
    X,
    Y,
    Z,
    All,
}

pub struct Billboard {
    pub mode: EBillboardMod,
}