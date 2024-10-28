use std::collections::{hash_map::Entry, HashMap};

use types::{
    core::{Value, ValueType},
    lang::{
        Expr, ExprInner, FuncCallExpr, FunctionSignature, Ident, IfExpr, LetExpr, LetExprDefinition,
    },
};

use crate::{error::Error, exec::ExecScope, function::Function};

enum EvalOrExecScope<'a> {
    EvalScope(&'a EvalScope<'a>),
    ExecScope(&'a ExecScope<'a>),
}

pub struct EvalScope<'a> {
    items: HashMap<Ident, Value>,
    parent_scope: EvalOrExecScope<'a>,
}

impl<'a> EvalScope<'a> {
    pub fn from(exec_scope: &'a ExecScope<'a>) -> Self {
        Self {
            items: HashMap::new(),
            parent_scope: EvalOrExecScope::ExecScope(exec_scope),
        }
    }

    pub fn get_value(&self, name: &Ident) -> Option<Value> {
        let maybe_ans = self.items.get(name).cloned();
        if maybe_ans.is_some() {
            maybe_ans
        } else if let EvalOrExecScope::EvalScope(parent) = self.parent_scope {
            parent.get_value(name)
        } else {
            None
        }
    }

    pub fn get_exec_scope(&self) -> &ExecScope {
        let mut scope = self;
        loop {
            match scope.parent_scope {
                EvalOrExecScope::EvalScope(eval_scope) => {
                    scope = eval_scope;
                }
                EvalOrExecScope::ExecScope(exec_scope) => {
                    return exec_scope;
                }
            }
        }
    }

    pub fn get_func(&self, sign: &FunctionSignature) -> Option<Function> {
        self.get_exec_scope().get_func(sign)
    }

    pub fn push(&'a self) -> EvalScope<'a> {
        EvalScope {
            items: HashMap::new(),
            parent_scope: EvalOrExecScope::EvalScope(self),
        }
    }

    pub fn insert_value(&mut self, name: Ident, value: Value) -> Result<(), Error> {
        match self.items.entry(name.clone()) {
            Entry::Occupied(_) => Err(Error::VariableRedefinition(name)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }
}

pub type EvalResult = Result<Value, Error>;

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
            .ok_or(Error::UndefinedVariable(self.clone()))
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
            .ok_or(Error::UndefinedFunction(sign))?;

        func.eval(arg_values, scope.get_exec_scope())
    }
}

impl Eval for IfExpr {
    fn eval(&self, scope: &EvalScope) -> EvalResult {
        for case in self.cases.iter() {
            match case.condition.eval(scope)? {
                Value::Bool(Some(true)) => return case.value.eval(scope),
                Value::Bool(Some(false)) => {}
                Value::Bool(None) => return Err(Error::UnexpectedNone),
                val => {
                    return Err(Error::UnexpectedType {
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
            Err(Error::NotingMatched)
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
                    return Err(Error::UnexpectedType {
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

#[cfg(test)]
pub fn eval(expr: &str) -> Value {
    use crate::eval::{Eval, EvalScope};

    let exec_scope = ExecScope::new();
    let eval_scope = EvalScope::from(&exec_scope);

    parser::expr(expr).unwrap().eval(&eval_scope).unwrap()
}

#[cfg(test)]
mod test {
    use types::core::Pt;

    use super::*;

    #[test]
    fn value() {
        assert_eq!(eval("2"), 2.into());
    }

    #[test]
    fn func_call() {
        assert_eq!(eval("1 + 1"), 2.into());
        assert_eq!(eval("pt 1.0 2.0"), Pt { x: 1.0, y: 2.0 }.into());
    }

    #[test]
    fn let_() {
        // Simple
        assert_eq!(eval("let x = 1 in x"), 1.into());

        // Multiple definitions
        assert_eq!(eval("let x = 1, y = 2 in x + y"), 3.into());

        // Complex expr
        assert_eq!(
            eval(
                r#"let
                x = 2,
                y = 3 * x,
                z = x + y,
            in z"#
            ),
            8.into()
        );
    }

    #[test]
    fn if_() {
        // Simple
        assert_eq!(eval("if 1 == 1 then 1 else 2"), 1.into());
        assert_eq!(eval("if 1 == 2 then 1 else 2"), 2.into());

        // Multiple cases
        assert_eq!(eval("if 1 == 1 then 1, 1 == 2 then 2 else 3"), 1.into());
        assert_eq!(eval("if 2 == 1 then 1, 2 == 2 then 2 else 3"), 2.into());
        assert_eq!(eval("if 3 == 1 then 1, 3 == 2 then 2 else 3"), 3.into());
    }
}
