use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::{VariableIndex, Value};

#[derive(Debug)]
pub struct Scope {
    // functions: HashMap<FunctionIndex>
    variables: HashMap<VariableIndex, Rc<dyn Value>>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Scope>>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn insert_variable(&mut self, index: VariableIndex, value: Rc<dyn Value>) -> bool {
        self.variables.try_insert(index, value).is_ok()
    }
}

impl Scope {
    pub fn find(&self, idx: &VariableIndex) -> Option<Rc<dyn Value>> {
        self.variables.get(idx).map_or(
            self.parent.as_ref().map_or(None, |parent| parent.borrow().find(idx)),
            |res| Some(res.clone())
        )
    }
}
