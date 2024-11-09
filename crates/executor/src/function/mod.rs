use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use types::{
    core::{Ident, Value, ValueType},
    lang::{FunctionDefinition, FunctionSignature},
};

use crate::{
    cexpr::{
        compile::{CScope, Compile},
        eval::{Eval, EvalResult, VarsMap},
        CExpr,
    },
    exec::{ExecError, ExecResult, ExecScope},
};

mod builtins;

pub(crate) type FuncMap = HashMap<FunctionSignature, Function>;

#[derive(Clone)]
pub(crate) struct Function(Arc<FunctionInner>);

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name} {args} -> {ret}",
            name = self.sign().name,
            args = self
                .sign()
                .arg_types
                .iter()
                .map(|arg_type| arg_type.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            ret = self.return_type()
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
    pub(crate) fn eval(&self, args: Vec<Value>) -> EvalResult {
        // TODO: check arg_types if #[cfg(debug)]
        match self
            .0
            .kind
            .get()
            .expect("dummy function cannot be evaluated")
        {
            FunctionInnerKind::BuiltIn(builtin) => builtin(args),
            FunctionInnerKind::CustomFunction(custom) => custom.eval(args),
        }
        // TODO: check return type if #[cfg(debug)]
    }

    pub(crate) fn sign(&self) -> FunctionSignature {
        self.0.sign.clone()
    }

    pub(crate) fn return_type(&self) -> ValueType {
        self.0.return_type.clone()
    }

    pub(crate) fn push_from_definition(
        def: FunctionDefinition,
        scope: &mut ExecScope,
    ) -> ExecResult {
        let FunctionDefinition {
            name,
            args,
            return_type,
            body,
        } = def;

        let (arg_names, arg_types): (Vec<_>, Vec<_>) = args
            .iter()
            .cloned()
            .map(|arg| (arg.name, arg.value_type))
            .unzip();

        // -------------------- Body --------------------
        let sign = FunctionSignature { name, arg_types };

        // Push dummy to allow recursive call in function body
        let func = Function::from(FunctionInner {
            sign: sign.clone(),
            return_type: return_type.clone(),
            kind: OnceLock::new(),
        });

        scope.insert_func(func.clone())?;
        let mut cscope = CScope::new(scope);
        for arg in args {
            cscope.insert_var_type(arg.name, arg.value_type)?;
        }

        let body = body.compile(&cscope)?;

        // -------------------- Args --------------------
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

        // -------------------- Return Type --------------------
        if body.value_type() != return_type {
            return Err(ExecError::UnexpectedType {
                expected: return_type,
                got: body.value_type(),
            });
        }

        func.0
            .kind
            .set(FunctionInnerKind::CustomFunction(CustomFunction {
                arg_names,
                body,
            }))
            .expect("initialization");

        Ok(())
    }
}

struct FunctionInner {
    sign: FunctionSignature,
    return_type: ValueType,

    /// If kind is empty, then function is considered to be "dummy". Dummy function is pushed to
    /// scope before function body is parsed. It is used to make recursion possible. Function with
    /// this kind will panic if evaluated.
    kind: OnceLock<FunctionInnerKind>,
}

enum FunctionInnerKind {
    BuiltIn(Box<dyn Sync + Send + 'static + Fn(Vec<Value>) -> EvalResult>),
    CustomFunction(CustomFunction),
}

impl Debug for FunctionInnerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionInnerKind::BuiltIn(_) => write!(f, "BUILTIN FUNCTION"),
            FunctionInnerKind::CustomFunction(_) => write!(f, "CUSTOM FUNCTION"),
        }
    }
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

#[cfg(test)]
mod test {
    use crate::exec::Exec;

    use super::*;

    #[test]
    fn simple() {
        let mut scope = ExecScope::new();
        parser::statement("sq x:int -> int = x * x")
            .unwrap()
            .exec(&mut scope)
            .unwrap();
        assert_eq!(
            scope
                .get_func(&FunctionSignature {
                    name: Ident::from("sq"),
                    arg_types: vec![ValueType::Int]
                })
                .unwrap()
                .return_type(),
            ValueType::Int,
        );
    }

    #[test]
    fn recursion() {
        let mut scope = ExecScope::new();
        parser::script(
            r#"
            fact x:int -> int = if
                x == 0 then 1,
                else x * fact (x - 1)

            f0 = fact 0
            f1 = fact 1
            f5 = fact 5
        "#,
        )
        .unwrap()
        .exec(&mut scope)
        .unwrap();

        assert_eq!(
            scope.get_node(&Ident::from("f0")).unwrap().get_value(),
            1.into()
        );
        assert_eq!(
            scope.get_node(&Ident::from("f1")).unwrap().get_value(),
            1.into()
        );
        assert_eq!(
            scope.get_node(&Ident::from("f5")).unwrap().get_value(),
            120.into()
        );
    }
}
