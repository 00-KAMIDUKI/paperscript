use std::cell::RefCell;
use std::rc::Rc;

use crate::{expr::Expr, value::Value, Type, error::RuntimeError, frame::Frame};

pub type BinaryOp = fn(&dyn Expr, &dyn Expr, Rc<RefCell<Frame>>) -> Result<Box<dyn Value>, RuntimeError>;


macro_rules! define_binary_op {
    ($name: ident $op: tt $($Type: ident $as_type: ident)*) => {
        pub fn $name(lhs: &dyn Expr, rhs: &dyn Expr, frame: Rc<RefCell<Frame>>) -> Result<Box<dyn Value>, RuntimeError> {
            let value1 = lhs.evaluate(frame.clone())?;
            let value2 = rhs.evaluate(frame.clone())?;
            if value1.type_() != value2.type_() { Err(RuntimeError::TypeError { current: value2.type_().clone(), expected: vec![value1.type_().clone()] }) } else {
                match value1.type_() {
                    $(Type::$Type => Ok(Box::new(value1.$as_type().unwrap() $op value2.$as_type().unwrap())),)*
                    _ => Err(RuntimeError::TypeError { current: value1.type_().clone(), expected: vec![$(Type::$Type,)*] })
                }
            }
        }
    }
}

macro_rules! define_numeric_ops {
    ($($name: ident $op: tt)*) => {
        $(define_binary_op!{
            $name $op
            Int64 as_i64
            Float64 as_f64
        })*
    }
}

define_numeric_ops!{
    add + 
    sub -
    mul *
    div /
    mod_ %
    gt >
    lt <
    ge >=
    le <=
}

pub fn eq(lhs: &dyn Expr, rhs: &dyn Expr, frame: Rc<RefCell<Frame>>) -> Result<Box<dyn Value>, RuntimeError> {
    let value1 = lhs.evaluate(frame.clone())?;
    let value2 = rhs.evaluate(frame.clone())?;
    if value1.type_() != value2.type_() { Err(RuntimeError::TypeError { current: value2.type_().clone(), expected: vec![value1.type_().clone()] }) } 
    else {
        match value1.type_() {
            Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() == value2.as_i64().unwrap())),
            Type::Float64 => Ok(Box::new(value1.as_f64().unwrap() == value2.as_f64().unwrap())),
            Type::Bool => Ok(Box::new(value1.as_bool().unwrap() == value2.as_bool().unwrap())),
            Type::Function(..) => unimplemented!(),
            _ => Err(RuntimeError::TypeError { current: value1.type_().clone(), expected: vec![] }),
        }
    }
}

pub fn ne(lhs: &dyn Expr, rhs: &dyn Expr, frame: Rc<RefCell<Frame>>) -> Result<Box<dyn Value>, RuntimeError> {
    let value1 = lhs.evaluate(frame.clone())?;
    let value2 = rhs.evaluate(frame.clone())?;
    if value1.type_() != value2.type_() { Err(RuntimeError::TypeError { current: value2.type_().clone(), expected: vec![value1.type_().clone()] }) } 
    else {
        match value1.type_() {
            Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() != value2.as_i64().unwrap())),
            Type::Float64 => Ok(Box::new(value1.as_f64().unwrap() != value2.as_f64().unwrap())),
            Type::Bool => Ok(Box::new(value1.as_bool().unwrap() != value2.as_bool().unwrap())),
            Type::Function(..) => unimplemented!(),
            _ => Err(RuntimeError::TypeError { current: value1.type_().clone(), expected: vec![] }),
        }
    }
}

