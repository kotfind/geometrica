use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};

use types::{
    core::{Value, ValueType},
    lang::{FunctionDefinition, FunctionSignature, Ident},
};

use crate::{
    cexpr::{
        compile::{CScope, Compile},
        eval::{Eval, EvalResult, VarsMap},
        CExpr,
    },
    exec::{ExecError, ExecScope},
};

mod builtins;

pub type FuncMap = HashMap<FunctionSignature, Function>;

#[derive(Clone)]
pub struct Function(Arc<FunctionInner>);

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

    pub fn sign(&self) -> FunctionSignature {
        self.0.sign.clone()
    }

    pub fn return_type(&self) -> ValueType {
        self.0.return_type.clone()
    }

    pub fn from_definition(def: FunctionDefinition, scope: &ExecScope) -> Result<Self, ExecError> {
        let FunctionDefinition {
            name,
            args,
            return_type,
            body,
        } = def;

        let mut cscope = CScope::new(scope);
        for arg in &args {
            cscope.insert_var_type(arg.name.clone(), arg.value_type.clone())?;
        }
        let body = body.compile(&cscope)?;

        let (arg_names, arg_types): (Vec<_>, Vec<_>) = args
            .into_iter()
            .map(|arg| (arg.name, arg.value_type))
            .unzip();

        let sign = FunctionSignature { name, arg_types };

        // Check for unprovided arguments
        let arg_names_set: HashSet<_> = arg_names.iter().collect();
        for required_var in body.required_vars() {
            if !arg_names_set.contains(required_var) {
                return Err(ExecError::UndefinedVariableInFunction {
                    var: required_var.clone(),
                    func: sign,
                });
            }
        }

        // Check for unused arguments
        for arg_name in &arg_names_set {
            if !body.required_vars().contains(arg_name) {
                // TODO: use better warning processing
                println!("WARN: unused variable {arg_name}");
            }
        }

        // Check return type
        if body.value_type() != return_type {
            return Err(ExecError::UnexpectedType {
                expected: return_type,
                got: body.value_type(),
            });
        }

        Ok(Function::from(FunctionInner {
            sign,
            return_type,
            kind: FunctionInnerKind::CustomFunction(CustomFunction { arg_names, body }),
        }))
    }
}

struct FunctionInner {
    sign: FunctionSignature,
    return_type: ValueType,
    kind: FunctionInnerKind,
}

enum FunctionInnerKind {
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
