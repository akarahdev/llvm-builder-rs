use std::collections::HashSet;
use crate::module::{CallingConvention, LinkageType};
use crate::values::Value;

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    attributes: HashSet<FunctionAttribute>
}

#[derive(Eq, Hash, Clone, PartialEq, Debug)]
pub enum FunctionAttribute {
    Linkage(LinkageType),
    CallConv(CallingConvention)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::functions::{Function, FunctionAttribute};
    use crate::module::{CallingConvention, LinkageType};
    use crate::values::Value;

    #[test]
    fn example_func() {
        let mut attrs = HashSet::new();
        attrs.insert(FunctionAttribute::CallConv(CallingConvention::C));
        attrs.insert(FunctionAttribute::Linkage(LinkageType::Private));
        let func = Function {
            name: "main".to_string(),
            attributes: attrs
        };
    }
}