use crate::{Expr, Value, Scope, VariableIndex};
use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct BinaryExpr {
    lhs: Box<dyn Expr>,
    op: fn(&dyn Expr, &dyn Expr) -> Result<Box<dyn Value>, ()>,
    rhs: Box<dyn Expr>,
}

impl Expr for BinaryExpr {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        self.op.call((self.lhs.as_ref(), self.rhs.as_ref())).map(Rc::from)
    }
}

#[derive(Debug)]
pub struct LetBinding {
    name: String,
    scope: Rc<RefCell<Scope>>,
    bind_expr: Box<dyn Expr>,
    in_expr: Box<dyn Expr>,
}

impl Expr for LetBinding {
    fn evaluate(&self) -> Result<Rc<dyn Value>, ()> {
        let res = self.scope.borrow_mut().variables.try_insert(VariableIndex{
            name: self.name.clone(),
        }, self.bind_expr.evaluate()?).is_ok();
        match res {
            true => self.in_expr.evaluate(),
            false => Err(()),
        }
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
        name: "a".to_string(),
        scope: scope.clone(),
        bind_expr: Box::new(1),
        in_expr: Box::new(BinaryExpr {
            lhs: Box::new(1),
            op: crate::bin_op::add,
            rhs: Box::new(crate::Variable { scope: scope.clone(), index: VariableIndex { name: "a".to_string() } })
        }),
    };
    assert_eq!(bind1.evaluate().unwrap().as_i64().unwrap(), 2)
}

