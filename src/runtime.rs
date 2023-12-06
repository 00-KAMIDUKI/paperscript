use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::{expr::VariableIndex, value::Value};

#[derive(Debug, Clone)]
pub struct Runtime {
    pub frame: Rc<RefCell<Frame>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            frame: Rc::new(RefCell::new(Frame::new()))
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub variables: HashMap<VariableIndex, Rc<dyn Value>>,
    parent: Option<Rc<RefCell<Frame>>>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Frame>>) -> Self {
        Frame {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn insert_variable(&mut self, index: VariableIndex, value: Rc<dyn Value>) -> bool {
        self.variables.try_insert(index, value).is_ok()
    }
}

impl Frame {
    pub fn find(&self, idx: &VariableIndex) -> Option<Rc<dyn Value>> {
        self.variables.get(idx).map_or(
            self.parent.as_ref().map_or(None, |parent| parent.borrow().find(idx)),
            |res| Some(res.clone())
        )
    }
}
