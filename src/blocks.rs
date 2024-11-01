use crate::inst::Instruction;

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub name: Label,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub struct Label {
    pub name: String,
}