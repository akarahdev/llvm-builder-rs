use crate::convert::ConvertIr;
use crate::inst::Instruction;

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub name: Label,
    pub instructions: Vec<Instruction>,
}

impl ConvertIr for Vec<BasicBlock> {
    fn ir(&self) -> String {
        self.iter()
            .map(|x| x.ir())
            .collect()
    }
}

impl ConvertIr for BasicBlock {
    fn ir(&self) -> String {
        format!("{}:\n{}",
                self.name.name,
                self.instructions
                    .iter()
                    .map(|x| format!("    {}", x.ir()))
                    .collect::<Vec<_>>()
                    .join("\n"))
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    pub name: String,
}