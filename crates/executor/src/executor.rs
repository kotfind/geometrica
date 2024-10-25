use std::collections::{hash_map::Entry, HashMap};

use thiserror::Error;
use types::{
    core::{Value, ValueType},
    lang::{
        Expr, ExprInner, FuncCallExpr, FunctionSignature, Ident, IfExpr, LetExpr, LetExprDefinition,
    },
};

use crate::function::Function;

pub struct EvalScope<'a> {
    items: HashMap<Ident, Value>,
    funcs: HashMap<FunctionSignature, Function>,
    parent_scope: Option<&'a EvalScope<'a>>,
}

impl<'a> EvalScope<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            funcs: HashMap::new(),
            parent_scope: None,
        }
    }

    pub fn get_value(&self, name: &Ident) -> Option<Value> {
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
        let maybe_ans = self.funcs.get(sign).cloned();
        if maybe_ans.is_some() {
            maybe_ans
        } else if let Some(parent) = self.parent_scope {
            parent.get_func(sign)
        } else {
            None
        }
    }

    pub fn push(&'a self) -> EvalScope<'a> {
        EvalScope {
            items: HashMap::new(),
            funcs: HashMap::new(),
            parent_scope: Some(self),
        }
    }

    pub fn insert_value(&mut self, name: Ident, value: Value) -> Result<(), EvalError> {
        match self.items.entry(name.clone()) {
            Entry::Occupied(_) => Err(EvalError::VariableRedefinition(name)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("variable '{0}' undefined")]
    UndefinedVariable(Ident),

    #[error("function '{0:?}' undefined")]
    UndefinedFunction(FunctionSignature),

    #[error("unexpected type for {_for}: expected {expected}, got {got}")]
    UnexpectedType {
        _for: String,
        expected: ValueType,
        got: ValueType,
    },

    #[error("got unexpected none value")]
    UnexpectedNone,

    #[error("if: no cases matched, default value not provided")]
    NotingMatched,

    #[error("Redefinition of variable '{0}'")]
    VariableRedefinition(Ident),
}

pub type EvalResult = Result<Value, EvalError>;

pub trait Eval {
    fn eval(&self, scope: &EvalScope) -> EvalResult;
}

impl Eval for Expr {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        match &self.0 as &ExprInner {
            ExprInner::Value(e) => e.eval(scope),
            ExprInner::Variable(e) => e.eval(scope),
            ExprInner::FuncCall(e) => e.eval(scope),
            ExprInner::If(e) => e.eval(scope),
            ExprInner::Let(e) => e.eval(scope),
        }
    }
}

impl Eval for Value {
    fn eval(&self, _scope: &EvalScope) -> EvalResult {
        Ok(self.clone())
    }
}

impl Eval for Ident {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        scope
            .get_value(self)
            .ok_or(EvalError::UndefinedVariable(self.clone()))
    }
}

impl Eval for FuncCallExpr {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        let arg_values: Vec<Value> = self
            .args
            .iter()
            .map(|arg| arg.eval(scope))
            .collect::<Result<_, _>>()?;

        let arg_types: Vec<ValueType> = arg_values.iter().map(|arg| arg.value_type()).collect();

        let sign = FunctionSignature {
            name: self.name.clone(),
            arg_types,
        };

        let func = scope
            .get_func(&sign)
            .ok_or(EvalError::UndefinedFunction(sign))?;

        func.eval(arg_values)
    }
}

impl Eval for IfExpr {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        for case in self.cases.iter() {
            match case.condition.eval(scope)? {
                Value::Bool(Some(true)) => return case.value.eval(scope),
                Value::Bool(Some(false)) => {}
                Value::Bool(None) => return Err(EvalError::UnexpectedNone),
                val => {
                    return Err(EvalError::UnexpectedType {
                        _for: "if condition".to_string(),
                        expected: ValueType::Bool,
                        got: val.value_type(),
                    })
                }
            }
        }
        if let Some(default_case_value) = &self.default_case_value {
            default_case_value.eval(scope)
        } else {
            Err(EvalError::NotingMatched)
        }
    }
}

impl Eval for LetExpr {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        let mut new_scope = scope.push();
        for LetExprDefinition {
            name,
            value_type,
            body,
        } in self.definitions.iter()
        {
            let value = body.eval(&new_scope)?;
            if let Some(value_type) = value_type {
                if &value.value_type() != value_type {
                    return Err(EvalError::UnexpectedType {
                        _for: name.to_string(),
                        expected: value_type.clone(),
                        got: value.value_type(),
                    });
                }
            }
            new_scope.insert_value(name.clone(), value)?;
        }

        self.body.eval(&new_scope)
    }
}
