use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;

use crate::error::RuntimeError;
use crate::value::{Value, Function};
use crate::{Frame, Type};
use crate::bin_op::BinaryOp;

pub trait Expr: Debug {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError>;
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<dyn Expr>,
    pub op: BinaryOp,
    pub rhs: Box<dyn Expr>,
}

impl Expr for BinaryExpr {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        self.op.call((self.lhs.as_ref(), self.rhs.as_ref(), frame)).map(Rc::from)
    }
}

#[derive(Debug)]
pub struct CondExpr {
    pub inner: Vec<Box<dyn Expr>>,
}

impl Expr for CondExpr {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        self.inner.chunks(2)
            .take(self.inner.len() / 2)
            .map(|slice| (&slice[0], &slice[1]))
            .map(|(cond, expr)| (cond.evaluate(frame.clone()), expr))
            .find_map(|(cond, expr)| match cond {
                Ok(cond) => match cond.as_bool() {
                    Some(value) => if value { Some(expr.evaluate(frame.clone())) } else { None },
                    None => Some(Err(RuntimeError::TypeError { current: cond.type_().clone(), expected: vec![Type::Bool] })),
                },
                Err(err) => Some(Err(err)),
            })
        .unwrap_or_else(|| self.inner.last().unwrap().evaluate(frame.clone()))
    }
}

#[derive(Debug)]
pub struct LetBinding {
    pub identifier: String,
    pub bind_expr: Box<dyn Expr>,
    pub in_expr: Box<dyn Expr>,
}

impl Expr for LetBinding {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        let value = self.bind_expr.evaluate(frame.clone())?;
        let res = frame.borrow_mut().insert_variable(
            VariableIndex::Name(self.identifier.clone()),
            value,
        );
        match res {
            true => self.in_expr.evaluate(frame.clone()),
            false => Err(RuntimeError::MultiDefined { identifier: self.identifier.clone() }),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum VariableIndex {
    Name(String),
    ParamIndex(usize),
}

impl From<&str> for VariableIndex {
    fn from(value: &str) -> Self {
        VariableIndex::Name(value.to_string())
    }
}

#[derive(Debug)]
pub struct Variable {
    pub index: VariableIndex,
}

impl Expr for Variable {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        let res = frame.borrow().find(&self.index);
        res.map_or(Err(RuntimeError::Undefined { index: self.index.clone() }), |res| Ok(res))
    }
}

#[derive(Debug)]
pub struct Invocation {
    params: Vec<Box<dyn Expr>>,
}

impl Expr for Invocation {
    fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
        println!("{:?}", &frame.borrow().variables);
        let function = self.params.first().unwrap().evaluate(frame.clone()).unwrap().as_function().unwrap().clone();
        let mut new_frame = Frame::from_parent(frame.clone());
        for (idx, param) in self.params.iter().enumerate() {
            let value = param.evaluate(frame.clone())?;
            new_frame.insert_variable(VariableIndex::ParamIndex(idx), value);
        }
        function.expr.evaluate(Rc::new(RefCell::new(new_frame)))
    }
}

#[test]
fn test_binary_expression() {
    let frame = Rc::new(RefCell::new(Frame::new()));
    assert_eq!(BinaryExpr {
        lhs: Box::new(3),
        op: crate::bin_op::add,
        rhs: Box::new(2),
    }.evaluate(frame).unwrap().as_i64().unwrap(), 5);
}

#[test]
fn test_let_binding() {
    let frame = Rc::new(RefCell::new(Frame::new()));
    let bind1 = LetBinding {
        identifier: "a".to_string(),
        bind_expr: Box::new(1),
        in_expr: Box::new(BinaryExpr {
            lhs: Box::new(1),
            op: crate::bin_op::add,
            rhs: Box::new(Variable { index: "a".into() })
        }),
    };
    assert_eq!(bind1.evaluate(frame).unwrap().as_i64().unwrap(), 2)
}

#[test]
fn test_conditional_expression() {
    let frame = Rc::new(RefCell::new(Frame::new()));
    assert_eq!(CondExpr {
        inner: vec![
            Box::new(false), Box::new(1),
            Box::new(true), Box::new(2),
            Box::new(3),
        ]
    }.evaluate(frame).unwrap().as_i64().unwrap(), 2);
}

#[test]
fn test_conditional_expression_lazy_evaluation() {
    #[derive(Debug, Clone, Copy)]
    struct A {
        evaluated: *mut bool,
    }

    impl Expr for A {
        fn evaluate(&self, frame: Rc<RefCell<Frame>>) -> Result<Rc<dyn Value>, RuntimeError> {
            unsafe {
                *self.evaluated = true;
            }
            Ok(Rc::new(1))
        }
    }

    let frame = Rc::new(RefCell::new(Frame::new()));
    let mut evaluated = false;
    let flag = A { evaluated: &mut evaluated };
    let expr = CondExpr {
        inner: vec![
            Box::new(true), Box::new(1),
            Box::new(false), Box::new(flag),
            Box::new(flag),
        ]
    };
    let res = expr.evaluate(frame);
    assert_eq!(res.unwrap().as_i64().unwrap(), 1);
    assert!(!evaluated);
}

#[test]
fn test_function() {
    use crate::bin_op;
    let frame = Rc::new(RefCell::new(Frame::new()));
    let function = Function {
        type_: vec![],
        expr: Rc::new(BinaryExpr {
            lhs: Box::new(Variable { index: VariableIndex::ParamIndex(1) }),
            op: bin_op::add,
            rhs: Box::new(1),
        }),
    };
    let invocation = Invocation {
        params: vec![Box::new(function), Box::new(1)],
    };
    assert_eq!(invocation.evaluate(frame).unwrap().as_i64().unwrap(), 2);
}

#[test]
fn test_recursion() {
    // fn if $1 > 10 then $1 else $0 ($1 + 1) end end
    use crate::bin_op;
    let frame = Rc::new(RefCell::new(Frame::new()));
    let function = Function {
        type_: vec![],
        expr: Rc::new(CondExpr { inner: vec![
            Box::new(BinaryExpr {
                lhs: Box::new(Variable { index: VariableIndex::ParamIndex(1) }),
                op: bin_op::gt,
                rhs: Box::new(10),
            }),
            Box::new(Variable { index: VariableIndex::ParamIndex(1) }),
            Box::new(Invocation { params: vec![
                Box::new(Variable { index: VariableIndex::ParamIndex(0) }),
                Box::new(BinaryExpr {
                    lhs: Box::new(Variable { index: VariableIndex::ParamIndex(1) }),
                    op: bin_op::add,
                    rhs: Box::new(1),
                }),
            ] })
        ] }),
    };
    let invocation = Invocation {
        params: vec![Box::new(function), Box::new(1)],
    };
    assert_eq!(invocation.evaluate(frame).unwrap().as_i64().unwrap(), 11);
}
