use crate::blocks::BasicBlock;
use crate::convert::ConvertIr;

#[derive(Clone, Debug)]
pub enum Type {
    /// The void type does not represent any value and has no size.
    Void,
    /// The function type can be thought of as a function signature.
    /// It consists of a return type and a list of formal parameter types.
    /// The return type of a function type is a void type or first class type — except for
    /// label and metadata types.
    FunctionType(Box<Type>, Vec<Type>),
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
    Struct(Vec<Type>),
    /// Opaque structure types are used to represent structure types that do not have a body
    /// specified.
    Opaque
}

impl ConvertIr for Type {
    fn ir(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::FunctionType(ty, body) => format!(
                "{}({})",
                ty.ir(),
                body.iter().map(|t| t.ir()).collect::<Vec<_>>().join(", ")
            ),
            Type::Integer(i) => format!("i{}", i),
            Type::Float16 => "half".to_string(),
            Type::BFloat16 => "bfloat".to_string(),
            Type::Float32 => "float".to_string(),
            Type::Float64 => "double".to_string(),
            Type::X86Float80 => "x86_fp80".to_string(),
            Type::Float128 => "fp128".to_string(),
            Type::PPCFloat128 => "ppc_fp128".to_string(),
            Type::X86AMX => "x86_amx".to_string(),
            Type::Ptr => "ptr".to_string(),
            Type::Target => "target(\"label\")".to_string(),
            Type::Vector { ty, size } => format!("[{} x {}]", ty.ir(), size),
            Type::Label => "label".to_string(),
            Type::Token => "token".to_string(),
            Type::Metadata => "metadata".to_string(),
            Type::Array(ty) => format!("[{}]", ty.ir()),
            Type::Struct(body) => format!("struct {}", body.iter().map(|t| t.ir()).collect::<Vec<_>>().join(", ")),
            Type::Opaque => "opaque".to_string()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    /// A global value, e.g @main
    GlobalVariable(GlobalPtr),
    /// A local register, e.g %2
    Register(RegisterData),
    /// A raw string, e.g c"Hello world!\00"
    CString(String),
    /// Represents a constant value
    Constant(ConstantData)
}

impl ConvertIr for Value {
    fn ir(&self) -> String {
        match self {
            Value::GlobalVariable(g) => g.ir(),
            Value::Register(r) => r.ir(),
            Value::Constant(c) => c.ir(),
            Value::CString(str) => format!("c{:?}", str)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConstantData {
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

impl ConvertIr for ConstantData {
    fn ir(&self) -> String {
        match self {
            ConstantData::True => "true".to_string(),
            ConstantData::False => "false".to_string(),
            ConstantData::Integer(i) => i.to_string(),
            ConstantData::FloatingPoint(f) => f.to_string(),
            ConstantData::NullPtr => "null".to_string(),
            ConstantData::StructureConstant(c) => format!(
                "{{ {} }}",
                c.iter().map(|x| format!("{} {}", x.0.ir(), x.1.ir())).collect::<Vec<_>>().join(", ")
            ),
            ConstantData::ArrayConstant(c) => format!(
                "[ {} ]",
                c.iter().map(|x| format!("{} {}", x.0.ir(), x.1.ir())).collect::<Vec<_>>().join(", ")
            ),
            ConstantData::VectorConstant(c) => format!(
                "< {} >",
                c.iter().map(|x| format!("{} {}", x.0.ir(), x.1.ir())).collect::<Vec<_>>().join(", ")
            ),
            ConstantData::Poison => "poison".to_string(),
            ConstantData::Undef => "undef".to_string(),
            ConstantData::BlockAddress(str, addr) => format!("blockaddress(@{}, %{})", str, addr)
        }
    }
}


#[derive(Clone, Debug)]
pub struct GlobalPtr {
    pub name: String,
}

impl ConvertIr for GlobalPtr {
    fn ir(&self) -> String {
        format!("@{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct RegisterData {
    pub name: String
}

impl RegisterData {
    pub fn new(name: &str) -> RegisterData {
       RegisterData {
           name: name.to_string()
       }
    }
}

impl ConvertIr for RegisterData {
    fn ir(&self) -> String {
        format!("%{}", self.name)
    }
}