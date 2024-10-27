use std::sync::Arc;

use types::{
    core::Value,
    lang::{Expr, FunctionSignature, Ident},
};

use crate::eval::{Eval, EvalResult, EvalScope};

mod builtin;

#[derive(Clone)]
pub struct Function(Arc<FunctionInner>);

struct FunctionInner {
    signature: FunctionSignature,
    kind: FunctionKind,
}

impl Function {
    pub fn eval(&self, args: Vec<Value>) -> EvalResult {
        let inner = &self.0;
        match &inner.kind {
            FunctionKind::BuiltIn(builtin) => builtin(args),
            FunctionKind::CustomFunction(custom) => custom.eval(&inner.signature, args),
        }
    }
}

enum FunctionKind {
    BuiltIn(Box<dyn Sync + Send + 'static + Fn(Vec<Value>) -> EvalResult>),
    CustomFunction(CustomFunction),
}

struct CustomFunction {
    arg_names: Vec<Ident>,
    body: Expr,
}

impl CustomFunction {
    fn eval(&self, sign: &FunctionSignature, args: Vec<Value>) -> EvalResult {
        assert!(self.arg_names.len() == sign.arg_types.len());
        assert!(self.arg_names.len() == args.len());

        let mut scope = EvalScope::new();
        for (arg_name, arg_value) in self.arg_names.iter().zip(args.into_iter()) {
            scope.insert_value(arg_name.clone(), arg_value)?;
        }

        self.body.eval(&scope)
    }
}
