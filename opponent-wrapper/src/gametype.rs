
pub trait GameType {
    const IS_VS_HUMAN: bool;
}

pub struct VsHuman;
pub struct VsComputer;

impl GameType for VsHuman {
    const IS_VS_HUMAN: bool = true;
}

impl GameType for VsComputer {
    const IS_VS_HUMAN: bool = false;
}