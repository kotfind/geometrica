use once_cell::sync::Lazy;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, OnceLock},
};
use types::{
    core::Ident,
    core::{Value, ValueType},
    lang::FunctionSignature,
};

use super::{FuncMap, Function, FunctionInner, FunctionInnerKind};
use crate::cexpr::eval::{EvalError, EvalResult};

mod cmp;
mod logic;
mod math;
mod struct_ops;
mod type_casts;

impl Function {
    pub(crate) fn get_builtin(sign: &FunctionSignature) -> Option<Function> {
        BUILT_IN_FUNCS.get(sign).cloned()
    }

    pub(crate) fn list_builtins() -> Vec<FunctionSignature> {
        BUILT_IN_FUNCS.keys().cloned().collect()
    }
}

// Unwraps value or returns EvalError::UnexpectedNone
macro_rules! unwrap_none {
    ($($var:ident),+) => {
        $(
            let $var = match $var {
                Some(v) => v,
                None => {
                    return Err(EvalError::UnexpectedNone);
                }
            };
        )*
    };
}
use unwrap_none;

// Inserts pair (FunctionSignature, Function) into $builtin_functions HashMap for function
// with name $name and specified arguments, return type and body
macro_rules! builtin {
    (INTO $builtin_functions:ident INSERT) => {};

    (INTO $builtin_functions:ident INSERT
        fn $name:literal ($($arg_name:ident : $arg_type:ident),*) -> $ret_type:ident $body:block
        $($rest:tt)*
    ) => {
        {
            let sign = FunctionSignature {
                name: Ident::from($name),
                arg_types: vec![
                    $(ValueType::$arg_type),*
                ]
            };

            let kind_inner = FunctionInnerKind::BuiltIn(Box::new(move |args: Vec<Value>| -> EvalResult {
                    let mut args_iter = args.into_iter();
                    $(
                        let $arg_name = match args_iter.next() {
                            Some(Value::$arg_type(v)) => v,
                            Some(_) => unreachable!("type should be as specified in signature"),
                            None => unreachable!("too few arguments provided"),
                        };
                    )*
                    assert!(args_iter.next().is_none());

                    let res = Value::from({$body}?);
                    assert!(matches!(res, Value::$ret_type(_)));
                    Ok(res)
                }));

            let kind = OnceLock::new();
            kind.set(kind_inner).expect("initialization");

            let func = Function(Arc::new(FunctionInner {
                sign: sign.clone(),
                return_type: ValueType::$ret_type,
                kind,
            }));

            match $builtin_functions.entry(sign.clone()) {
                Entry::Occupied(_) => panic!("redefinition of {sign}"),
                Entry::Vacant(e) => e.insert(func),
            };

            builtin!(INTO $builtin_functions INSERT $($rest)*);
        }
    };
}
use builtin;

// Same as builtin, but all arguments are unwrapped and result is Ok-ed
macro_rules! simple_builtin {
    (INTO $builtin_functions:ident INSERT) => {};

    (INTO $builtin_functions:ident INSERT
        fn $name:literal ($($arg_name:ident : $arg_type:ident),*) -> $ret_type:ident $body:block
        $($rest:tt)*
    ) => {
        builtin!(INTO $builtin_functions INSERT fn $name ($($arg_name : $arg_type),*) -> $ret_type {
            unwrap_none!($($arg_name),*);
            Ok($body)
        });
        simple_builtin!(INTO $builtin_functions INSERT $($rest)*);
    };
}
use simple_builtin;

static BUILT_IN_FUNCS: Lazy<FuncMap> = Lazy::new(|| {
    // TODO: check for overflow
    let mut builtins = HashMap::new();

    math::populate(&mut builtins);
    cmp::populate(&mut builtins);
    logic::populate(&mut builtins);
    type_casts::populate(&mut builtins);
    struct_ops::populate(&mut builtins);

    builtins
});
