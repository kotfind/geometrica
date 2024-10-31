use std::collections::{hash_map, HashMap, HashSet};

use thiserror::Error;
use types::{
    core::ValueType,
    lang::{
        Command, Definition, FunctionDefinition, FunctionSignature, Ident, Statement,
        ValueDefinition,
    },
};

use crate::{
    compile::{CError, CScope, Compile},
    eval::{Eval, EvalError},
    function::{CustomFunction, FuncMap, Function, FunctionInner, FunctionInnerKind},
    node::{CExprNode, Node, NodeInnerKind},
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
    fn insert_func(&mut self, func: Function) -> ExecResult {
        match self.funcs.entry(func.0.sign.clone()) {
            hash_map::Entry::Occupied(_) => {
                Err(ExecError::FunctionRedefinition(func.0.sign.clone()))
            }
            hash_map::Entry::Vacant(e) => {
                e.insert(func);
                Ok(())
            }
        }
    }

    fn insert_node(&mut self, name: Ident, node: Node) -> ExecResult {
        match self.nodes.entry(name.clone()) {
            hash_map::Entry::Occupied(_) => Err(ExecError::VariableRedefinition(name)),
            hash_map::Entry::Vacant(e) => {
                e.insert(node);
                Ok(())
            }
        }
    }

    fn get_node(&self, name: &Ident) -> Option<Node> {
        self.nodes.get(name).cloned()
    }
}

pub type ExecResult = Result<(), ExecError>;

pub trait Exec {
    fn exec(self, scope: &mut ExecScope) -> ExecResult;
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

        let body = body.compile(&CScope::new() /* XXX: provide actual scope */)?;

        if let Some(expected_type) = value_type {
            if body.0.value_type != expected_type {
                return Err(ExecError::UnexpectedType {
                    expected: expected_type,
                    got: body.0.value_type.clone(),
                });
            }
        }

        if body.0.required_vars.is_empty() {
            let node = Node::from(NodeInnerKind::Value(body.eval(&HashMap::new())?));
            scope.insert_node(name, node)?;
        } else {
            let bindings: Vec<(Ident, Node)> = body
                .0
                .required_vars
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
            let node = Node::from(NodeInnerKind::CExpr(CExprNode {
                bindings: bindings.clone(),
                body,
            }));

            scope.insert_node(name, node)?;

            for (_, binding_node) in bindings {
                binding_node
                    .0
                    .lock()
                    .unwrap()
                    .required_by
                    .push(binding_node.downgrade());
            }
        }

        Ok(())
    }
}

impl Exec for FunctionDefinition {
    fn exec(self, scope: &mut ExecScope) -> ExecResult {
        let FunctionDefinition {
            name,
            args,
            return_type,
            body,
        } = self;

        let body = body.compile(&CScope::new() /* XXX: provide actual scope */)?;

        let (arg_names, arg_types): (Vec<_>, Vec<_>) = args
            .into_iter()
            .map(|arg| (arg.name, arg.value_type))
            .unzip();

        let sign = FunctionSignature { name, arg_types };

        // Check for unprovided arguments
        let arg_names_set: HashSet<_> = arg_names.iter().collect();
        for required_var in &body.0.required_vars {
            if !arg_names_set.contains(required_var) {
                return Err(ExecError::UndefinedVariableInFunction {
                    var: required_var.clone(),
                    func: sign,
                });
            }
        }

        // Check for unused arguments
        for arg_name in &arg_names_set {
            if !body.0.required_vars.contains(arg_name) {
                // TODO: use better warning processing
                println!("WARN: unused variable {arg_name}");
            }
        }

        // Check return type
        if body.0.value_type != return_type {
            return Err(ExecError::UnexpectedType {
                expected: return_type,
                got: body.0.value_type.clone(),
            });
        }

        let func = Function::from(FunctionInner {
            sign,
            return_type,
            kind: FunctionInnerKind::CustomFunction(CustomFunction { arg_names, body }),
        });

        scope.insert_func(func)
    }
}
