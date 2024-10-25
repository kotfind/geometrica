use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use types::{
    core::{Value, ValueType},
    lang::{Expr, FunctionSignature, Ident},
};

use crate::executor::{Eval, EvalResult, EvalScope};

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

    pub fn get_builtin(sign: &FunctionSignature) -> Option<Function> {
        assert!(sign.name.0.starts_with('#'));

        static BUILT_IN_FUNCS: Lazy<HashMap<FunctionSignature, Function>> = Lazy::new(|| {
            let mut ans: HashMap<FunctionSignature, Function> = HashMap::new();

            // XXX: Need better syntax for built-in function definition (possibly macro)
            let sign = FunctionSignature {
                name: Ident::from("#add"),
                arg_types: vec![ValueType::Int, ValueType::Int],
            };
            ans.insert(
                sign.clone(),
                Function(Arc::new(FunctionInner {
                    signature: sign,
                    kind: FunctionKind::BuiltIn(Box::new(|args: Vec<Value>| {
                        let Value::Int(Some(lhs)) = args[0] else {
                            panic!("Fix Me")
                        };
                        let Value::Int(Some(rhs)) = args[1] else {
                            panic!("Fix Me")
                        };
                        Ok(Value::Int(Some(lhs + rhs)))
                    })),
                })),
            );
            ans
        });

        BUILT_IN_FUNCS.get(sign).cloned()
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
