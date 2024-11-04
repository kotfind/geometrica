use std::collections::{hash_map, HashMap};

use thiserror::Error;
use types::{
    core::ValueType,
    lang::{
        Command, Definition, FunctionDefinition, FunctionSignature, Ident, Statement,
        ValueDefinition,
    },
};

use crate::{
    cexpr::{
        compile::{CError, CScope, Compile},
        eval::{Eval, EvalError},
    },
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
}

pub struct ExecScope {
    funcs: FuncMap,
    nodes: HashMap<Ident, Node>,
}

impl ExecScope {
    pub fn new() -> Self {
        Self {
            funcs: FuncMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        let ans = Function::get_builtin(sign);
        if ans.is_some() {
            return ans;
        }

        self.funcs.get(sign).cloned()
    }

    pub fn insert_func(&mut self, func: Function) -> ExecResult {
        match self.funcs.entry(func.sign()) {
            hash_map::Entry::Occupied(_) => Err(ExecError::FunctionRedefinition(func.sign())),
            hash_map::Entry::Vacant(e) => {
                e.insert(func);
                Ok(())
            }
        }
    }

    pub fn insert_node(&mut self, name: Ident, node: Node) -> ExecResult {
        match self.nodes.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => Err(ExecError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(e) => {
                e.insert(node);
                Ok(())
            }
        }
    }

    pub fn get_node(&self, name: &Ident) -> Option<Node> {
        self.nodes.get(name).cloned()
    }
}

pub type ExecResult = Result<(), ExecError>;

pub trait Exec {
    fn exec(self, scope: &mut ExecScope) -> ExecResult;
}

impl Exec for Vec<Statement> {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        // TODO: nested scope; recover on error
        for stmt in self {
            stmt.exec(scope)?;
        }
        Ok(())
    }
}

impl Exec for Statement {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        match self {
            Statement::Definition(def) => def.exec(scope),
            Statement::Command(cmd) => cmd.exec(scope),
        }
    }
}

impl Exec for Command {
    fn exec(self, _scope: &mut ExecScope) -> ExecResult {
        todo!()
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
        let ValueDefinition {
            name,
            value_type,
            body,
        } = self;

        let body = body.compile(&CScope::new(scope))?;

        if let Some(expected_type) = value_type {
            if body.value_type() != expected_type {
                return Err(ExecError::UnexpectedType {
                    expected: expected_type,
                    got: body.value_type(),
                });
            }
        }

        let node = if body.required_vars().is_empty() {
            Node::from_value(body.eval(&HashMap::new())?)
        } else {
            let bindings: Vec<(Ident, Node)> = body
                .required_vars()
                .iter()
                .map(|var| {
                    (
                        var.clone(),
                        scope
                            .get_node(var)
                            .expect("var should be defined as body was successfully compiled"),
                    )
                })
                .collect();

            Node::from_cexpr(body, bindings.clone())?
        };

        scope.insert_node(name, node)?;

        Ok(())
    }
}

impl Exec for FunctionDefinition {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        scope.insert_func(Function::from_definition(self, scope)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod function_definition {
        use super::*;

        #[test]
        fn simple() {
            let mut scope = ExecScope::new();
            parser::statement("sq x:int -> int = x * x")
                .unwrap()
                .exec(&mut scope)
                .unwrap();
            assert_eq!(
                scope
                    .get_func(&FunctionSignature {
                        name: Ident::from("sq"),
                        arg_types: vec![ValueType::Int]
                    })
                    .unwrap()
                    .return_type(),
                ValueType::Int,
            );
        }
    }

    mod value_definition {
        use super::*;

        #[test]
        fn value() {
            let mut scope = ExecScope::new();
            parser::statement("x = 1")
                .unwrap()
                .exec(&mut scope)
                .unwrap();
            assert_eq!(
                scope.get_node(&Ident::from("x")).unwrap().get_value(),
                1.into()
            );
        }

        #[test]
        fn complex_value() {
            let mut scope = ExecScope::new();
            parser::statement("x = 1 + 1")
                .unwrap()
                .exec(&mut scope)
                .unwrap();
            assert_eq!(
                scope.get_node(&Ident::from("x")).unwrap().get_value(),
                2.into()
            );
        }

        #[test]
        fn type_assert() {
            let mut scope = ExecScope::new();
            assert!(matches!(
                parser::statement("x:real = 1").unwrap().exec(&mut scope),
                Err(ExecError::UnexpectedType {
                    expected: ValueType::Real,
                    got: ValueType::Int
                })
            ));
        }
    }
}
