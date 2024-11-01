use std::collections::HashSet;
use crate::convert::ConvertIr;
use crate::functions::Function;

struct Module {
    functions: Vec<Function>
}

#[derive(Eq, Hash, Clone, PartialEq, Debug)]
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

impl ConvertIr for LinkageType {
    fn ir(&self) -> String {
        match self {
            LinkageType::Private => "private",
            LinkageType::Internal => "internal",
            LinkageType::AvailableExternally => "available_externally",
            LinkageType::LinkOnce => "link_once",
            LinkageType::Weak => "weak",
            LinkageType::Common => "common",
            LinkageType::Appending => "appending",
            LinkageType::ExternWeak => "extern_weak",
            LinkageType::LinkOnceODR => "link_once_odr",
            LinkageType::WeakODR => "weak_odr",
            LinkageType::External => "external",
        }.to_string()
    }
}

#[derive(Eq, Hash, Clone, PartialEq, Debug)]
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

impl ConvertIr for CallingConvention {
    fn ir(&self) -> String {
        match self {
            CallingConvention::C => "ccc".to_string(),
            CallingConvention::Fast => "fastcc".to_string(),
            CallingConvention::Cold => "coldcc".to_string(),
            CallingConvention::Ghc => "ghccc".to_string(),
            CallingConvention::C11 => "cc 11".to_string(),
            CallingConvention::AnyReg => "any_regcc".to_string(),
            CallingConvention::PreserveMost => "preserve_mostcc".to_string(),
            CallingConvention::PreserveAll => "preserve_allcc".to_string(),
            CallingConvention::PreserveNone => "preserve_nonecc".to_string(),
            CallingConvention::CxxFastTLS => "cxx_fast_tls_cc".to_string(),
            CallingConvention::Tail => "tailcc".to_string(),
            CallingConvention::Swift => "swiftcc".to_string(),
            CallingConvention::SwiftTail => "swift_tailcc".to_string(),
            CallingConvention::WindowsControlFlowGuard => "cfguard_checkcc".to_string(),
            CallingConvention::Numbered(n) => format!("cc {}", n)
        }
    }
}