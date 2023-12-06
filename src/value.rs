use std::cell::RefCell;
use std::{rc::Rc, fmt::Display};

use crate::type_::Type;
use crate::error::RuntimeError;
use crate::expr::Expr;
use crate::frame::Frame;

pub trait Value: Expr + Display {
    fn type_(&self) -> Type;
    fn as_i64(&self) -> Option<i64> { None }
    fn as_f64(&self) -> Option<f64> { None }
    fn as_bool(&self) -> Option<bool> { None }
    fn as_function(&self) -> Option<&Function> { None }
}

impl<T: Value + Clone + 'static> Expr for T {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        Ok(Rc::new(self.clone()))
    }
}

impl Value for f64 {
    fn type_(&self) -> Type {
        Type::Float64
    }
    fn as_f64(&self) -> Option<f64> { Some(*self) }
}

impl Value for i64 {
    fn type_(&self) -> Type {
        Type::Int64
    }
    fn as_i64(&self) -> Option<i64> { Some(*self) }
}

impl Value for bool {
    fn type_(&self) -> Type {
        Type::Bool
    }
    fn as_bool(&self) -> Option<bool> { Some(*self) }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub type_: Vec<Type>,
    pub expr: Rc<dyn Expr>,
}

impl Value for Function {
    fn type_(&self) -> Type { Type::Function(self.type_.clone()) }

    fn as_function(&self) -> Option<&Function> {
        Some(self)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

#[derive(Clone, Debug)]
pub struct Undefined;

impl Value for Undefined {
    fn type_(&self) -> Type { Type::Undefined }
}

impl Display for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undefined")
    }
}


