use std::collections::HashSet;
use crate::blocks::BasicBlock;
use crate::convert::ConvertIr;
use crate::module::{CallingConvention, LinkageType};
use crate::values::{RegisterData, Type, Value};

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    return_type: Type,
    parameters: Vec<(Type, RegisterData)>,
    blocks: Vec<BasicBlock>,

    linkage: Option<LinkageType>,
    calling_convention: Option<CallingConvention>,
}

impl ConvertIr for Function {
    fn ir(&self) -> String {
        let blocks = self.blocks.clone();
        if blocks.is_empty() {
            format!("declare {} {} @{}()",
                    self.linkage.clone().map(|x| x.ir()).unwrap_or_else(String::new),
                           self.return_type.clone().ir(),
                           self.name.clone())
        } else {
            format!("define {} {} @{} {{\n{}\n}}",
                    self.linkage.clone().map(|x| x.ir()).unwrap_or_else(String::new),
                    self.return_type.clone().ir(),
                    self.name.clone(),
                    blocks.iter().map(|x| x.ir()).collect::<Vec<_>>().join("\n"),
            )
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::{BasicBlock, Label};
    use crate::convert::ConvertIr;
    use crate::functions::Function;
    use crate::inst::Instruction;
    use crate::module::{LinkageType};
    use crate::values::{ConstantData, RegisterData, Type, Value};

    #[test]
    fn example_func() {
        let func = Function {
            name: "main".to_string(),
            return_type: Type::Integer(32),
            parameters: vec![],
            linkage: Some(LinkageType::Private),
            calling_convention: None,
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

        println!("Function:\n{}", func.ir());
    }
}