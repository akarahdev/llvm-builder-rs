use std::collections::HashSet;
use crate::blocks::BasicBlock;
use crate::module::{CallingConvention, LinkageType};
use crate::values::Value;

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    attributes: HashSet<FunctionAttribute>,
    blocks: Vec<BasicBlock>
}

#[derive(Eq, Hash, Clone, PartialEq, Debug)]
pub enum FunctionAttribute {
    Linkage(LinkageType),
    CallConv(CallingConvention)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::blocks::{BasicBlock, Label};
    use crate::functions::{Function, FunctionAttribute};
    use crate::inst::Instruction;
    use crate::module::{CallingConvention, LinkageType};
    use crate::values::{Type, Value};

    #[test]
    fn example_func() {
        let mut attrs = HashSet::new();
        attrs.insert(FunctionAttribute::CallConv(CallingConvention::C));
        attrs.insert(FunctionAttribute::Linkage(LinkageType::Private));

        let func = Function {
            name: "main".to_string(),
            attributes: attrs,
            blocks: vec![
                BasicBlock {
                    name: Label { name: "entry".to_string() },
                    instructions: vec![
                        Instruction::Ret(Type::Integer(16), Value::Integer(10)),
                    ],
                }
            ]
        };
    }
}