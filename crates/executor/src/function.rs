use std::{collections::HashMap, sync::Arc};

use types::{
    core::{Value, ValueType},
    lang::{Expr, FunctionSignature, Ident},
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
    pub kind: FunctionKind,
    pub return_type: ValueType,
}

// impl Function {
//     pub fn eval(&self, args: Vec<Value>) -> EvalResult {
//         todo!()
//         // let inner = &self.0;
//         // match &inner.kind {
//         //     FunctionKind::BuiltIn(builtin) => builtin(args),
//         //     FunctionKind::CustomFunction(custom) => custom.eval(&inner.signature, args, exec_scope),
//         // }
//     }
// }

enum FunctionKind {
    // BuiltIn(Box<dyn Sync + Send + 'static + Fn(Vec<Value>) -> EvalResult>),
    // CustomFunction(CustomFunction),
}

struct CustomFunction {
    arg_names: Vec<Ident>,
    body: Expr,
}

// impl CustomFunction {
//     fn eval(&self, sign: &FunctionSignature, args: Vec<Value>) -> EvalResult {
//         todo!()
//         // assert!(self.arg_names.len() == sign.arg_types.len());
//         // assert!(self.arg_names.len() == args.len());
//         //
//         // let mut scope = EvalScope::from(&exec_scope);
//         // for (arg_name, arg_value) in self.arg_names.iter().zip(args.into_iter()) {
//         //     scope.insert_value(arg_name.clone(), arg_value)?;
//         // }
//         //
//         // self.body.eval(&scope)
//     }
// }
