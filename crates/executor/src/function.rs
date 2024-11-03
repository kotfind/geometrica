use std::{collections::HashMap, fmt::Debug, sync::Arc};

use types::{
    core::{Value, ValueType},
    lang::{FunctionSignature, Ident},
};

use crate::{
    cexpr::CExpr,
    eval::{Eval, EvalResult, VarsMap},
};

mod builtins;

pub type FuncMap = HashMap<FunctionSignature, Function>;

#[derive(Clone)]
pub struct Function(pub Arc<FunctionInner>);

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name} {args} -> {ret}",
            name = self.0.sign.name,
            args = self
                .0
                .sign
                .arg_types
                .iter()
                .map(|arg_type| arg_type.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            ret = self.0.return_type
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl From<FunctionInner> for Function {
    fn from(inner: FunctionInner) -> Self {
        Self(Arc::new(inner))
    }
}

impl Function {
    pub fn eval(&self, args: Vec<Value>) -> EvalResult {
        // TODO: check arg_types if #[cfg(debug)]
        let inner = &self.0;
        match &inner.kind {
            FunctionInnerKind::BuiltIn(builtin) => builtin(args),
            FunctionInnerKind::CustomFunction(custom) => custom.eval(args),
        }
        // TODO: check return type if #[cfg(debug)]
    }
}

pub struct FunctionInner {
    pub sign: FunctionSignature,
    pub return_type: ValueType,
    pub kind: FunctionInnerKind,
}

pub enum FunctionInnerKind {
    BuiltIn(Box<dyn Sync + Send + 'static + Fn(Vec<Value>) -> EvalResult>),
    CustomFunction(CustomFunction),
}

pub struct CustomFunction {
    pub arg_names: Vec<Ident>,
    pub body: CExpr,
}

impl CustomFunction {
    fn eval(&self, args: Vec<Value>) -> EvalResult {
        assert!(self.arg_names.len() == args.len());
        let vars: VarsMap = self.arg_names.clone().into_iter().zip(args).collect();
        self.body.eval(&vars)
    }
}
