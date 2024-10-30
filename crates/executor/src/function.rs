use std::{collections::HashMap, sync::Arc};

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

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

pub struct FunctionInner {
    pub sign: FunctionSignature,
    pub return_type: ValueType,
    kind: FunctionKind,
}

impl Function {
    pub fn eval(&self, args: Vec<Value>) -> EvalResult {
        // TODO: check arg_types if #[cfg(debug)]
        let inner = &self.0;
        match &inner.kind {
            FunctionKind::BuiltIn(builtin) => builtin(args),
            FunctionKind::CustomFunction(custom) => custom.eval(args),
        }
        // TODO: check return type if #[cfg(debug)]
    }
}

enum FunctionKind {
    BuiltIn(Box<dyn Sync + Send + 'static + Fn(Vec<Value>) -> EvalResult>),
    CustomFunction(CustomFunction),
}

struct CustomFunction {
    arg_names: Vec<Ident>,
    body: CExpr,
}

impl CustomFunction {
    fn eval(&self, args: Vec<Value>) -> EvalResult {
        assert!(self.arg_names.len() == args.len());
        let vars: VarsMap = self.arg_names.clone().into_iter().zip(args).collect();
        self.body.eval(&vars)
    }
}
