#![feature(fn_traits, map_try_insert)]

use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Debug, PartialEq, Eq)]
enum Type {
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

trait Value: Expr + Display {
    fn type_(&self) -> &Type;
    fn as_i64(&self) -> Option<i64>;
    fn as_f64(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    // fn as_string(&self) -> Option<String>;
}

trait Expr: Debug {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()>;
}

mod value;
mod expr;
mod bin_op;
mod parser;

// enum FunctionIndex {
//     Name(String)
// }

// struct Function {
//     index: FunctionIndex,
//     type_: Type,
// }

#[derive(Debug, Hash, PartialEq, Eq)]
struct VariableIndex {
    name: String,
}

#[derive(Debug)]
struct Variable {
    index: VariableIndex,
    scope: Rc<RefCell<Scope>>,
}

#[derive(Debug)]
struct Scope {
    // functions: HashMap<FunctionIndex>
    variables: HashMap<VariableIndex, Rc<dyn Value>>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }
}

impl Scope {
    fn find(&self, idx: &VariableIndex) -> Option<Rc<dyn Value>> {
        self.variables.get(idx).map_or(
            self.parent.as_ref().map_or(None, |parent| parent.borrow().find(idx)),
            |res| Some(res.clone())
        )
    }
}

impl Expr for Variable {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        self.scope.borrow().find(&self.index).map_or(Err(()), |res| Ok(res))
    }
}

fn main() {
    // parser::parse_expr();
}

