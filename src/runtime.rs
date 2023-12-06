use std::{cell::RefCell, rc::Rc};

use crate::{frame::Frame, expr::Expr, value::Value, error::RuntimeError};

pub struct Runtime {
    frame: Rc<RefCell<Frame>>,
    main: Box<dyn Expr>,
}

impl Runtime {
    pub fn new(main: Box<dyn Expr>) -> Self {
        Self {
            frame: Rc::new(RefCell::new(Frame::new())),
            main,
        }
    }
    pub fn evaluate(&self) -> Result<Rc<dyn Value>, RuntimeError> {
        self.main.evaluate(self.frame.clone())
    }
}
