use crate::blocks::{BasicBlock, Label};
use crate::values::{Register, Type, Value};

#[derive(Clone, Debug)]
pub enum Instruction {
    Ret(Type, Value),
    RetVoid,

    Br(Value, Label, Label),
    Switch(Type, Value, Vec<SwitchBranch>),

    Unreachable,

    FNeg(Register, Type, Value),
    Add(Register, Type, Value, Value, ArithmeticFlags),
    FAdd(Register, Type, Value, Value, ArithmeticFlags),
    Sub(Register, Type, Value, Value, ArithmeticFlags),
    FSub(Register, Type, Value, Value, ArithmeticFlags),
    Mul(Register, Type, Value, Value, ArithmeticFlags),
    FMul(Register, Type, Value, Value, ArithmeticFlags),
    UDiv(Register, Type, Value, Value, ArithmeticFlags),
    SDiv(Register, Type, Value, Value, ArithmeticFlags),
    FDiv(Register, Type, Value, Value, ArithmeticFlags),
    URem(Register, Type, Value, Value, ArithmeticFlags),
    SRem(Register, Type, Value, Value, ArithmeticFlags),
    FRem(Register, Type, Value, Value, ArithmeticFlags),

    Shl(Register, Type, Value, Value, ArithmeticFlags),
    LShr(Register, Type, Value, Value, ArithmeticFlags),
    AShr(Register, Type, Value, Value, ArithmeticFlags),

    And(Register, Type, Value, Value, ArithmeticFlags),
    Or(Register, Type, Value, Value, ArithmeticFlags),
    Xor(Register, Type, Value, Value, ArithmeticFlags),

    ExtractElement(Register, Type, Value, Type, Value),
    InsertElement(Register, Type, Value, Type, Value),
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