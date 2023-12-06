use crate::{type_::Type, expr::VariableIndex};

pub enum Error {
    ParseError(),
    RuntimeError(RuntimeError)
}

#[derive(Debug)]
pub enum RuntimeError {
    TypeError {
        expected: Vec<Type>,
        current: Type,
    },
    MultiDefined {
        identifier: String,
    },
    Undefined {
        index: VariableIndex,
    }
}

