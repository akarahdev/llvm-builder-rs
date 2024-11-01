use crate::blocks::{BasicBlock, Label};
use crate::convert::ConvertIr;
use crate::values::{ConstantData, RegisterData, Type, Value};

#[derive(Clone, Debug)]
pub enum Instruction {
    Ret(Type, Value),
    RetVoid,

    Br(Value, Label, Label),
    Switch(Type, Value, Vec<SwitchBranch>),

    Unreachable,

    FNeg(RegisterData, Type, Value),
    Add(RegisterData, Type, Value, Value, ArithmeticFlags),
    FAdd(RegisterData, Type, Value, Value, ArithmeticFlags),
    Sub(RegisterData, Type, Value, Value, ArithmeticFlags),
    FSub(RegisterData, Type, Value, Value, ArithmeticFlags),
    Mul(RegisterData, Type, Value, Value, ArithmeticFlags),
    FMul(RegisterData, Type, Value, Value, ArithmeticFlags),
    UDiv(RegisterData, Type, Value, Value, ArithmeticFlags),
    SDiv(RegisterData, Type, Value, Value, ArithmeticFlags),
    FDiv(RegisterData, Type, Value, Value, ArithmeticFlags),
    URem(RegisterData, Type, Value, Value, ArithmeticFlags),
    SRem(RegisterData, Type, Value, Value, ArithmeticFlags),
    FRem(RegisterData, Type, Value, Value, ArithmeticFlags),

    Shl(RegisterData, Type, Value, Value, ArithmeticFlags),
    LShr(RegisterData, Type, Value, Value, ArithmeticFlags),
    AShr(RegisterData, Type, Value, Value, ArithmeticFlags),

    And(RegisterData, Type, Value, Value, ArithmeticFlags),
    Or(RegisterData, Type, Value, Value, ArithmeticFlags),
    Xor(RegisterData, Type, Value, Value, ArithmeticFlags),

    ExtractElement(RegisterData, Type, Value, Type, Value),
    InsertElement(RegisterData, Type, Value, Type, Value),

    ExtractValue(RegisterData, Type, Value, i32),
    InsertValue(RegisterData, Type, Value, i32),

    Alloca(RegisterData, Type),
    AllocaMultiple(RegisterData, Type, i32),
    Load(RegisterData, Value),
    Store(Value, Type, Value),
}

impl ConvertIr for Instruction {
    fn ir(&self) -> String {
        match self {
            Instruction::Ret(t, v) => format!("ret {} {}", t.ir(), v.ir()),
            Instruction::RetVoid => "ret void".to_string(),
            Instruction::Alloca(data, ty) => format!("{} = alloca {}", data.ir(), ty.ir()),
            _ => "TODO".to_string()
        }
    }
}
#[derive(Clone, Debug)]
pub struct SwitchBranch {
    ty: Type,
    condition: Value,
    label: Value
}

#[derive(Clone, Debug)]
pub enum ArithmeticFlags {
    WrapAllowed,
    NoUnsignedWrap,
    NoSignedWrap,
    NoSignedOrUnsignedWrap
}