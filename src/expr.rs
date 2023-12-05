use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;

use crate::error::RuntimeError;
use crate::value::{Value, Function};
use crate::{Scope, Type};
use crate::bin_op::BinaryOp;

pub trait Expr: Debug {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError>;
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<dyn Expr>,
    pub op: BinaryOp,
    pub rhs: Box<dyn Expr>,
}

impl Expr for BinaryExpr {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        self.op.call((self.lhs.as_ref(), self.rhs.as_ref())).map(Rc::from)
    }
}

#[derive(Debug)]
pub struct CondExpr {
    pub inner: Vec<Box<dyn Expr>>,
}

impl Expr for CondExpr {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        self.inner.chunks(2)
            .take(self.inner.len() / 2)
            .map(|slice| (&slice[0], &slice[1]))
            .map(|(cond, expr)| (cond.evaluate(), expr))
            .find_map(|(cond, expr)| match cond {
                Ok(cond) => match cond.as_bool() {
                    Some(value) => if value { Some(expr.evaluate()) } else { None },
                    None => Some(Err(RuntimeError::TypeError { current: cond.type_().clone(), expected: vec![Type::Bool] })),
                },
                Err(err) => Some(Err(err)),
            })
        .unwrap_or(self.inner.last().unwrap().evaluate())
    }
}

#[derive(Debug)]
pub struct LetBinding {
    pub identifier: String,
    pub scope: Rc<RefCell<Scope>>,
    pub bind_expr: Box<dyn Expr>,
    pub in_expr: Box<dyn Expr>,
}

impl Expr for LetBinding {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        let res = self.scope.borrow_mut().insert_variable(
            VariableIndex::from_name(self.identifier.clone()),
            self.bind_expr.evaluate()?,
        );
        match res {
            true => self.in_expr.evaluate(),
            false => Err(RuntimeError::MultiDefined { identifier: self.identifier.clone() }),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct VariableIndex {
    pub name: String,
    // pub param_idx: usize,
}

impl VariableIndex {
    pub fn from_name(name: String) -> Self {
        VariableIndex {
            name,
            // param_idx: usize::MAX,
        }
    }

    // pub fn from_param_idx(param_idx: usize) -> Self {
    //     VariableIndex {
    //         name: String::new(),
    //         param_idx,
    //     }
    // }
}

#[derive(Debug)]
pub struct Variable {
    pub index: VariableIndex,
    pub scope: Rc<RefCell<Scope>>,
}

impl Expr for Variable {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        let res = self.scope.borrow().find(&self.index);
        res.map_or(Err(RuntimeError::Undefined { index: self.index.clone() }), |res| Ok(res))
    }
}

#[derive(Debug)]
pub struct Invocation {
    function: Function,
    params: Vec<Box<dyn Expr>>,
}

impl Expr for Invocation {
    fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        self.function.expr.evaluate()
    }
}

#[test]
fn test_binary_expression() {
    assert_eq!(BinaryExpr {
        lhs: Box::new(3),
        op: crate::bin_op::add,
        rhs: Box::new(2),
    }.evaluate().unwrap().as_i64().unwrap(), 5);
}

#[test]
fn test_let_binding() {
    let scope = Rc::new(RefCell::new(Scope::new()));
    let bind1 = LetBinding {
        identifier: "a".to_string(),
        scope: scope.clone(),
        bind_expr: Box::new(1),
        in_expr: Box::new(BinaryExpr {
            lhs: Box::new(1),
            op: crate::bin_op::add,
            rhs: Box::new(Variable { scope: scope.clone(), index: VariableIndex::from_name("a".to_string()) })
        }),
    };
    assert_eq!(bind1.evaluate().unwrap().as_i64().unwrap(), 2)
}

#[test]
fn test_conditional_expression() {
    assert_eq!(CondExpr {
        inner: vec![
            Box::new(false), Box::new(1),
            Box::new(true), Box::new(2),
            Box::new(3),
        ]
    }.evaluate().unwrap().as_i64().unwrap(), 2);
}

