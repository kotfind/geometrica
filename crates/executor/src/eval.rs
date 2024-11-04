use std::collections::HashMap;

use thiserror::Error;
use types::{core::Value, lang::Ident};

use crate::cexpr::{CExpr, CExprInnerKind, FuncCallCExpr, IfCExpr};

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("unexpected none")]
    UnexpectedNone,

    #[error("none of if cases matched")]
    NotingMatched,
}

pub type EvalResult = Result<Value, EvalError>;
pub type VarsMap = HashMap<Ident, Value>;

pub trait Eval {
    fn eval(&self, vars: &VarsMap) -> EvalResult;
}

impl Eval for CExpr {
    fn eval(&self, vars: &HashMap<Ident, Value>) -> EvalResult {
        for required_var in &self.0.required_vars {
            assert!(vars.contains_key(required_var));
        }

        match &self.0.kind {
            CExprInnerKind::Value(e) => e.eval(vars),
            CExprInnerKind::Variable(e) => e.eval(vars),
            CExprInnerKind::FuncCall(e) => e.eval(vars),
            CExprInnerKind::If(e) => e.eval(vars),
        }
    }
}

impl Eval for Value {
    fn eval(&self, _vars: &VarsMap) -> EvalResult {
        Ok(self.clone())
    }
}

impl Eval for Ident {
    fn eval(&self, vars: &VarsMap) -> EvalResult {
        let ans = vars
            .get(self)
            .expect("all variables specified in required_vars should be defined");
        Ok(ans.clone())
    }
}

impl Eval for FuncCallCExpr {
    fn eval(&self, vars: &VarsMap) -> EvalResult {
        let arg_vals = self
            .args
            .iter()
            .map(|arg| arg.eval(vars))
            .collect::<Result<Vec<_>, _>>()?;
        // TODO: check arg types if #[cfg(debug)]
        let ans = self.func.eval(arg_vals)?;
        // TODO: check return type if #[cfg(debug)]
        Ok(ans)
    }
}

impl Eval for IfCExpr {
    fn eval(&self, vars: &VarsMap) -> EvalResult {
        for case in self.cases.iter() {
            match case.cond.eval(vars)? {
                Value::Bool(Some(true)) => return case.value.eval(vars),
                Value::Bool(Some(false)) => {}
                Value::Bool(None) => return Err(EvalError::UnexpectedNone),
                _ => unreachable!("type should have been checked when compiling"),
            }
        }

        if let Some(default_case_value) = &self.default_case_value {
            default_case_value.eval(vars)
        } else {
            Err(EvalError::NotingMatched)
        }
    }
}

#[cfg(test)]
pub fn eval(expr: &str) -> Value {
    use crate::{
        compile::{CScope, Compile},
        exec::ExecScope,
    };

    let expr = parser::expr(expr).expect("failed to parse");
    let cexpr = expr
        .compile(&CScope::new(&ExecScope::new()))
        .expect("failed to compile");
    cexpr.eval(&VarsMap::new()).expect("failed to eval")
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
