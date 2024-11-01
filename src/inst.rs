use crate::blocks::{BasicBlock, Label};
use crate::values::{Type, Value};

#[derive(Clone, Debug)]
pub enum Instruction {
    Ret(Type, Value),
    RetVoid,

    Br(Value, Label, Label),
    Switch { branches: Vec<SwitchBranch> },

    Unreachable,

    FNeg(Value, Type, Value),
    Add(Value, Type, Value, Value, ArithmeticFlags),
    FAdd(Value, Type, Value, Value, ArithmeticFlags),
    Sub(Value, Type, Value, Value, ArithmeticFlags),
    FSub(Value, Type, Value, Value, ArithmeticFlags),
    Mul(Value, Type, Value, Value, ArithmeticFlags),
    FMul(Value, Type, Value, Value, ArithmeticFlags),
    UDiv(Value, Type, Value, Value, ArithmeticFlags),
    SDiv(Value, Type, Value, Value, ArithmeticFlags),
    FDiv(Value, Type, Value, Value, ArithmeticFlags),
    URem(Value, Type, Value, Value, ArithmeticFlags),
    SRem(Value, Type, Value, Value, ArithmeticFlags),
    FRem(Value, Type, Value, Value, ArithmeticFlags),

    Shl(Value, Type, Value, Value, ArithmeticFlags),
    LShr(Value, Type, Value, Value, ArithmeticFlags),
    AShr(Value, Type, Value, Value, ArithmeticFlags),

    And(Value, Type, Value, Value, ArithmeticFlags),
    Or(Value, Type, Value, Value, ArithmeticFlags),
    Xor(Value, Type, Value, Value, ArithmeticFlags),

    ExtractElement(Value, Type, Value, Type, Value),
    InsertElement(Value, Type, Value, Type, Value),
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