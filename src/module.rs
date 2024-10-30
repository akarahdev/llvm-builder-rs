use std::collections::HashSet;
use crate::functions::Function;

struct Module {
    functions: Vec<Function>
}

pub enum LinkageType {
    Private,
    Internal,
    AvailableExternally,
    LinkOnce,
    Weak,
    Common,
    Appending,
    ExternWeak,
    LinkOnceODR,
    WeakODR,
    External
}

pub enum CallingConvention {
    C,
    Fast,
    Cold,
    Ghc,
    C11,
    AnyReg,
    PreserveMost,
    PreserveAll,
    PreserveNone,
    CxxFastTLS,
    Tail,
    Swift,
    SwiftTail,
    WindowsControlFlowGuard,
    Numbered(u64)
}