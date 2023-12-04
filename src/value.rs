use std::rc::Rc;

use crate::{Value, Type, Expr, error::RuntimeError};

impl<T: Value + Copy + 'static> Expr for T {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        Ok(Rc::new(self.clone()))
    }
}

impl Value for f64 {
    fn type_(&self) -> &Type {
        &Type::Float64
    }
    fn as_i64(&self) -> Option<i64> { None }
    fn as_f64(&self) -> Option<f64> { Some(*self) }
    fn as_bool(&self) -> Option<bool> { None }
}

impl Value for i64 {
    fn type_(&self) -> &Type {
        &Type::Int64
    }
    fn as_i64(&self) -> Option<i64> { Some(*self) }
    fn as_f64(&self) -> Option<f64> { None }
    fn as_bool(&self) -> Option<bool> { None }
}

impl Value for bool {
    fn type_(&self) -> &Type {
        &Type::Bool
    }
    fn as_i64(&self) -> Option<i64> { None }
    fn as_f64(&self) -> Option<f64> { None }
    fn as_bool(&self) -> Option<bool> { Some(*self) }
}

