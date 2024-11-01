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
    use crate::values::{ConstantData, RegisterData, Type, Value};
    use crate::values::Value::{Constant as ConstantValue, Register as RegisterValue};

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
                        Instruction::Alloca(RegisterData::new("1"), Type::Integer(32)),
                        Instruction::Store(Value::Register(RegisterData::new("1")), Type::Integer(32), Value::Constant(ConstantData::Integer(10))),
                        Instruction::Load(RegisterData::new("2"), Value::Register(RegisterData::new("1"))),
                        Instruction::Ret(Type::Integer(32), Value::Register(RegisterData::new("2")))
                    ],
                }
            ]
        };
    }
}