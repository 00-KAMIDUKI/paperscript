use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;

use crate::{Value, Scope, VariableIndex};
use crate::bin_op::BinaryOp;

pub trait Expr: Debug {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()>;
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Rc<dyn Expr>,
    pub op: BinaryOp,
    pub rhs: Rc<dyn Expr>,
}

impl Expr for BinaryExpr {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        self.op.call((self.lhs.as_ref(), self.rhs.as_ref())).map(Rc::from)
    }
}

#[derive(Debug)]
pub struct CondExpr {
    inner: Vec<Rc<dyn Expr>>,
}

impl Expr for CondExpr {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        self.inner.chunks(2)
            .take(self.inner.len() / 2)
            .map(|slice| (&slice[0], &slice[1]))
            .map(|(cond, expr)| (cond.evaluate(), expr))
            .find_map(|(cond, expr)| match cond {
                Ok(cond) => match cond.as_bool() {
                    Some(cond) => if cond { Some(expr.evaluate()) } else { None },
                    None => Some(Err(())),
                },
                Err(err) => Some(Err(err)),
            })
        .unwrap_or(self.inner.last().unwrap().evaluate())
    }
}

#[derive(Debug)]
pub struct LetBinding {
    pub name: String,
    pub scope: Rc<RefCell<Scope>>,
    pub bind_expr: Rc<dyn Expr>,
    pub in_expr: Rc<dyn Expr>,
}

impl Expr for LetBinding {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        let res = self.scope.borrow_mut().insert_variable(VariableIndex{
            name: self.name.clone(),
        }, self.bind_expr.evaluate()?);
        // let new_scope = Scope::from_parent(self.scope.clone());
        // *self.scope.borrow_mut() = new_scope;
        // self.scope.replace(Scope::from_parent(self.scope.clone()));
        match res {
            true => self.in_expr.evaluate(),
            false => Err(()),
        }
    }
}

#[test]
fn test_binary_expression() {
    assert_eq!(BinaryExpr {
        lhs: Rc::new(3),
        op: crate::bin_op::add,
        rhs: Rc::new(2),
    }.evaluate().unwrap().as_i64().unwrap(), 5);
}

#[test]
fn test_let_binding() {
    let scope = Rc::new(RefCell::new(Scope::new()));
    let bind1 = LetBinding {
        name: "a".to_string(),
        scope: scope.clone(),
        bind_expr: Rc::new(1),
        in_expr: Rc::new(BinaryExpr {
            lhs: Rc::new(1),
            op: crate::bin_op::add,
            rhs: Rc::new(crate::Variable { scope: scope.clone(), index: VariableIndex { name: "a".to_string() } })
        }),
    };
    assert_eq!(bind1.evaluate().unwrap().as_i64().unwrap(), 2)
}

#[test]
fn test_conditional_expression() {
    assert_eq!(CondExpr {
        inner: vec![
            Rc::new(false), Rc::new(1),
            Rc::new(true), Rc::new(2),
            Rc::new(3),
        ]
    }.evaluate().unwrap().as_i64().unwrap(), 2);
}

