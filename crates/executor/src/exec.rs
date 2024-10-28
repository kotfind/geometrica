use std::collections::{hash_map::Entry, HashMap};

use types::lang::{FunctionSignature, Ident};

use crate::{
    error::Error,
    function::{FuncMap, Function},
    node::Node,
};

pub struct ExecScope<'a> {
    funcs: FuncMap,
    items: HashMap<Ident, Node>,
    parent_scope: Option<&'a ExecScope<'a>>,
}

impl<'a> ExecScope<'a> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            items: HashMap::new(),
            parent_scope: None,
        }
    }

    pub fn get_value(&self, name: &Ident) -> Option<Node> {
        let maybe_ans = self.items.get(name).cloned();
        if maybe_ans.is_some() {
            maybe_ans
        } else if let Some(parent) = self.parent_scope {
            parent.get_value(name)
        } else {
            None
        }
    }

    pub fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        if let Some(func) = Function::get_builtin(sign) {
            return Some(func);
        }

        let maybe_ans = self.funcs.get(sign).cloned();
        if maybe_ans.is_some() {
            maybe_ans
        } else if let Some(parent) = self.parent_scope {
            parent.get_func(sign)
        } else {
            None
        }
    }

    pub fn push(&'a self) -> ExecScope<'a> {
        ExecScope {
            items: HashMap::new(),
            funcs: HashMap::new(),
            parent_scope: Some(self),
        }
    }

    pub fn insert_value(&mut self, name: Ident, value: Node) -> Result<(), Error> {
        match self.items.entry(name.clone()) {
            Entry::Occupied(_) => Err(Error::VariableRedefinition(name)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn insert_func(&mut self, sign: FunctionSignature, func: Function) -> Result<(), Error> {
        match self.funcs.entry(sign.clone()) {
            Entry::Occupied(_) => Err(Error::FunctionRedefinition(sign)),
            Entry::Vacant(entry) => {
                entry.insert(func);
                Ok(())
            }
        }
    }
}
