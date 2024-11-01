use crate::blocks::BasicBlock;

#[derive(Clone, Debug)]
pub enum Type {
    /// The integer type is a very simple type that simply specifies an arbitrary bit width for the
    /// integer type desired. Any bit width from 1 bit to 223(about 8 million) can be specified.
    Integer(i32),

    /// 16-bit floating-point value (IEEE-754 binary16)
    Float16,
    /// 16-bit “brain” floating-point value (7-bit significand). Provides the same number of
    /// exponent bits as float, so that it matches its dynamic range, but with greatly reduced
    /// precision. Used in Intel’s AVX-512 BF16 extensions and Arm’s ARMv8.6-A extensions,
    /// among others.
    BFloat16,
    /// 32-bit floating-point value (IEEE-754 binary32)
    Float32,
    /// 64-bit floating-point value (IEEE-754 binary64)
    Float64,
    /// 80-bit floating-point value (X87)
    X86Float80,
    /// 128-bit floating-point value (IEEE-754 binary128)
    Float128,
    /// 128-bit floating-point value (two 64-bits)
    PPCFloat128,
    /// Represents a value held in an AMX tile register on an x86 machine. The operations allowed
    /// on it are quite limited. Only few intrinsics are allowed: stride load and store,
    /// zero and dot product. No instruction is allowed for this type. There are no arguments,
    /// arrays, pointers, vectors or constants of this type.
    X86AMX,
    /// The pointer type ptr is used to specify memory locations. Pointers are commonly used to
    /// reference objects in memory.
    Ptr,
    /// Target extension types represent types that must be preserved through optimization, but are otherwise generally opaque to the compiler.
    Target,
    /// A vector type is a simple derived type that represents a vector of elements.
    /// Vector types are used when multiple primitive data are operated in parallel using a
    /// single instruction (SIMD).
    Vector { ty: Box<Type>, size: i32 },

    /// The label type represents code labels.
    Label,
    /// The token type is used when a value is associated with an instruction but all uses of the
    /// value must not attempt to introspect or obscure it.
    Token,
    /// The metadata type represents embedded metadata.
    Metadata,

    /// The array type is a very simple derived type that arranges elements sequentially in memory.
    Array(Box<Type>),
    /// The structure type is used to represent a collection of data members together in memory.
    Struct { members: Vec<Type> },
    /// Opaque structure types are used to represent structure types that do not have a body
    /// specified.
    Opaque
}

#[derive(Clone, Debug)]
pub enum Value {
    /// A global value, e.g @main
    GlobalVariable(GlobalPtr),
    /// A local register, e.g %2
    Register(Register),
    /// A raw string, e.g c"Hello world!\00"
    CString(String),
    /// Represents a constant value
    Constant(Constant)
}

#[derive(Clone, Debug)]
pub enum Constant {
    /// Represents an i1 'true'.
    True,
    /// Represents an i1 'false'.
    False,

    /// Represents an integer constant. Represented by 'i128' since that is Rust's
    /// limit.
    Integer(i128),

    /// Represents a floating-point constant. Represented by `f64' since that is Rust's
    /// limit.
    FloatingPoint(f64),

    /// Represents a null pointer.
    NullPtr,

    /// Represents a structure constant.
    StructureConstant(Vec<(Type, Value)>),
    /// Represents an array constant.
    ArrayConstant(Vec<(Type, Value)>),
    /// Represents an vector constant.
    VectorConstant(Vec<(Type, Value)>),

    /// A poison value is a result of an erroneous operation.
    /// Most instructions return ‘poison’ when one of their arguments is ‘poison’.
    /// It is correct to replace a poison value with an undef value or any value of the type.
    Poison,
    /// Indicates that the user of the value may receive an unspecified bit-pattern.
    /// Undefined values may be of any type (other than ‘label’ or ‘void’) and be used anywhere a
    /// constant is permitted.
    Undef,

    /// This constant computes the address of the specified basic block in the specified function.
    /// Taking the address of the entry block is illegal.
    BlockAddress(String, String)
}


#[derive(Clone, Debug)]
pub struct GlobalPtr {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Register {
    pub name: String
}