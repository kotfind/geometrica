use std::collections::{hash_map, HashMap};

use thiserror::Error;
use types::{
    core::{Ident, Value, ValueType},
    lang::{Definition, Expr, FunctionDefinition, FunctionSignature, ValueDefinition},
};

use crate::{
    cexpr::{compile::CError, eval::EvalError},
    compile::{CScope, Compile},
    eval::Eval,
    function::{FuncMap, Function},
    node::Node,
};

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("expr compilation error")]
    CompileError(#[from] CError),

    #[error("eval error")]
    EvalError(#[from] EvalError),

    #[error("undefined variable '{var}' in function '{func:?}'")]
    UndefinedVariableInFunction { var: Ident, func: FunctionSignature },

    #[error("function redefinition: {0:?}")]
    FunctionRedefinition(FunctionSignature),

    #[error("variable redefinition: {0}")]
    VariableRedefinition(Ident),

    #[error("unexpected type: expected {expected}, got {got}")]
    UnexpectedType { expected: ValueType, got: ValueType },

    #[error("variable '{0}' is not defined")]
    UndefinedVariable(Ident),

    #[error("cannot set! {old_type} variable '{name}' with {new_type} value")]
    SetDifferentType {
        name: Ident,
        old_type: ValueType,
        new_type: ValueType,
    },
}

pub struct ExecScope {
    funcs: FuncMap,
    nodes: HashMap<Ident, Node>,
}

impl Default for ExecScope {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecScope {
    pub fn new() -> Self {
        Self {
            funcs: FuncMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn delete(&mut self, name: Ident) -> Result<(), ExecError> {
        let node = self
            .get_node(&name)
            .ok_or(ExecError::UndefinedVariable(name.clone()))?;

        #[allow(clippy::mutable_key_type)]
        let nodes_to_delete = node.get_nodes_to_delete();

        self.nodes
            .retain(|_name, node| !nodes_to_delete.contains(node));

        Ok(())
    }

    pub fn set(&self, name: Ident, value: Value) -> Result<(), ExecError> {
        let node = self
            .get_node(&name)
            .ok_or(ExecError::UndefinedVariable(name.clone()))?;

        if node.value_type() != value.value_type() {
            return Err(ExecError::SetDifferentType {
                name,
                old_type: node.value_type(),
                new_type: value.value_type(),
            });
        }

        node.set(value)?;

        Ok(())
    }

    pub fn eval_expr(
        &self,
        expr: Expr,
        mut vars: HashMap<Ident, Value>,
    ) -> Result<Value, ExecError> {
        // TODO: create new exec scope and push vars there
        let mut cscope = CScope::new(self);
        for (name, value) in &vars {
            cscope.insert_var_type(name.clone(), value.value_type().clone())?;
        }

        let cexpr = expr.compile(&cscope)?;

        for var in cexpr.required_vars() {
            if let Some(node) = self.get_node(var) {
                vars.insert(var.clone(), node.get_value());
            }
        }

        Ok(cexpr.eval(&vars)?)
    }

    pub fn get_all_items(&self) -> HashMap<Ident, Value> {
        self.nodes
            .iter()
            .map(|(name, node)| (name.clone(), node.get_value()))
            .collect()
    }

    pub fn get_item(&self, name: &Ident) -> Option<Value> {
        self.get_node(name).map(|node| node.get_value())
    }

    pub(crate) fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        let ans = Function::get_builtin(sign);
        if ans.is_some() {
            return ans;
        }

        self.funcs.get(sign).cloned()
    }

    pub(crate) fn insert_func(&mut self, func: Function) -> ExecResult {
        match self.funcs.entry(func.sign()) {
            hash_map::Entry::Occupied(_) => Err(ExecError::FunctionRedefinition(func.sign())),
            hash_map::Entry::Vacant(e) => {
                e.insert(func);
                Ok(())
            }
        }
    }

    pub(crate) fn insert_node(&mut self, name: Ident, node: Node) -> ExecResult {
        match self.nodes.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => Err(ExecError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(e) => {
                e.insert(node);
                Ok(())
            }
        }
    }

    pub(crate) fn get_node(&self, name: &Ident) -> Option<Node> {
        self.nodes.get(name).cloned()
    }
}

pub type ExecResult = Result<(), ExecError>;

pub trait Exec {
    fn exec(self, scope: &mut ExecScope) -> ExecResult;
}

impl Exec for Vec<Definition> {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        // TODO: nested scope; recover on error
        for def in self {
            def.exec(scope)?;
        }
        Ok(())
    }
}

impl Exec for Definition {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        match self {
            Definition::ValueDefinition(val_def) => val_def.exec(scope),
            Definition::FunctionDefinition(var_def) => var_def.exec(scope),
        }
    }
}

impl Exec for ValueDefinition {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        scope.insert_node(self.name.clone(), Node::from_value_definition(self, scope)?)?;

        Ok(())
    }
}

impl Exec for FunctionDefinition {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        Function::push_from_definition(self, scope)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn definitions() {
        let mut scope = ExecScope::new();
        parser::definitions(
            r#"
            sq x:int -> int = x^2
            sq x:real -> real = x^2
            sum x:int y:int -> int = x + y
            a = 1
            b = 2
            c = sum (sq a) (sq b)
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        let node_names = ["a", "b", "c"];
        for node_name in node_names {
            assert!(scope.get_node(&Ident::from(node_name)).is_some());
        }

        let func_signs = [
            FunctionSignature {
                name: Ident::from("sq"),
                arg_types: vec![ValueType::Int],
            },
            FunctionSignature {
                name: Ident::from("sq"),
                arg_types: vec![ValueType::Real],
            },
            FunctionSignature {
                name: Ident::from("sum"),
                arg_types: vec![ValueType::Int, ValueType::Int],
            },
        ];
        for func_sign in func_signs {
            assert!(scope.get_func(&func_sign).is_some());
        }
    }

    #[test]
    fn get_all_items() {
        let mut scope = ExecScope::new();
        parser::definitions(
            r#"
            sq x:int -> int = x^2
            sq x:real -> real = x^2
            sum x:int y:int -> int = x + y
            a = 1
            b = 2
            c = sum (sq a) (sq b)
            "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        let all_items: HashMap<Ident, Value> = scope.get_all_items().into_iter().collect();
        let expected_items: HashMap<Ident, Value> = HashMap::from([
            (Ident::from("a"), 1.into()),
            (Ident::from("b"), 2.into()),
            (Ident::from("c"), 5.into()),
        ]);

        assert_eq!(all_items, expected_items);
    }
}
