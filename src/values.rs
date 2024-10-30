pub enum Value {
    /// A global value, e.g @main
    GlobalVariable(String),
    /// A local register, e.g %2
    Register(String),
    /// A cstring, e.g c"Hello world!\00"
    CString(String),
}