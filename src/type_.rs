use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Undefined,
    Float64,
    Int64,
    // String,
    Bool,
    Function(Vec<Type>),
    // Void,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        match self {
            Function(types) => {
                for typ in types.iter().rev().take(types.len() - 1) {
                    write!(f, "{}->", typ)?
                }
                write!(f, "{}", types.first().unwrap())
            },
            _ => write!(f, "{:?}", self),
        }
    }
}

