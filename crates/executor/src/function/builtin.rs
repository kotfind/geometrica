use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use types::{
    core::{Value, ValueType},
    lang::FunctionSignature,
    lang::Ident,
};

use super::{Function, FunctionInner, FunctionKind};
use crate::eval::{EvalError, EvalResult};

impl Function {
    pub fn get_builtin(sign: &FunctionSignature) -> Option<Function> {
        assert!(sign.name.0.starts_with('#'));
        BUILT_IN_FUNCS.get(sign).cloned()
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

// Inserts pair (FunctionSignature, Function) into builtin_functions HashMap for function
// with name '#' + $name and specified arguments, return type and body
macro_rules! builtin {
    ($name:ident ($($arg_name:ident : $arg_type:ident),*) -> $ret_type:ident $body:block) => {
        {
            let sign = FunctionSignature {
                name: Ident("#".to_string() + stringify!($name)),
                arg_types: vec![
                    $(ValueType::$arg_type),*
                ]
            };
            let func = Function(Arc::new(FunctionInner {
                signature: sign.clone(),
                kind: FunctionKind::BuiltIn(Box::new(move |args: Vec<Value>| -> EvalResult {
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
                })),
            }));
            (sign, func)
        }
    };
}

// Same as builtin, but all arguments are unwrapped and result is Ok-ed
macro_rules! simple_builtin {
    ($name:ident ($($arg_name:ident : $arg_type:ident),*) -> $ret_type:ident $body:block) => {
        builtin!($name ($($arg_name : $arg_type),*) -> $ret_type {
            unwrap_none!($($arg_name),*);
            Ok($body)
        })
    };
}

static BUILT_IN_FUNCS: Lazy<HashMap<FunctionSignature, Function>> = Lazy::new(|| {
    // TODO: check for overflow
    HashMap::from([
        // Add
        simple_builtin!(add (lhs:  Int, rhs:  Int) -> Int  { lhs        + rhs }),
        simple_builtin!(add (lhs: Real, rhs: Real) -> Real { lhs        + rhs }),
        simple_builtin!(add (lhs:  Int, rhs: Real) -> Real { lhs as f64 + rhs }),
        simple_builtin!(add (lhs: Real, rhs:  Int) -> Real { lhs        + rhs as f64 }),
        // Sub
        simple_builtin!(sub (lhs:  Int, rhs:  Int) -> Int  { lhs        - rhs }),
        simple_builtin!(sub (lhs: Real, rhs: Real) -> Real { lhs        - rhs }),
        simple_builtin!(sub (lhs:  Int, rhs: Real) -> Real { lhs as f64 - rhs }),
        simple_builtin!(sub (lhs: Real, rhs:  Int) -> Real { lhs        - rhs as f64 }),
        // Mul
        simple_builtin!(mul (lhs:  Int, rhs:  Int) -> Int  { lhs        * rhs }),
        simple_builtin!(mul (lhs: Real, rhs: Real) -> Real { lhs        * rhs }),
        simple_builtin!(mul (lhs:  Int, rhs: Real) -> Real { lhs as f64 * rhs }),
        simple_builtin!(mul (lhs: Real, rhs:  Int) -> Real { lhs        * rhs as f64 }),
        // Div
        simple_builtin!(div (lhs:  Int, rhs:  Int) -> Int  { lhs        / rhs }),
        simple_builtin!(div (lhs: Real, rhs: Real) -> Real { lhs        / rhs }),
        simple_builtin!(div (lhs:  Int, rhs: Real) -> Real { lhs as f64 / rhs }),
        simple_builtin!(div (lhs: Real, rhs:  Int) -> Real { lhs        / rhs as f64 }),
        // Pow
        simple_builtin!(div (lhs:  Int, rhs:  Int) -> Int  { lhs       .pow(rhs as u32 /* TODO: check if cast fails */) }),
        simple_builtin!(div (lhs: Real, rhs: Real) -> Real { lhs       .powf(rhs) }),
        simple_builtin!(div (lhs:  Int, rhs: Real) -> Real { (lhs as f64).powf(rhs) }),
        simple_builtin!(div (lhs: Real, rhs:  Int) -> Real { lhs       .powi(rhs as i32 /* TODO: check if cast fails */) }),
        // Rem
        simple_builtin!(rem (lhs:  Int, rhs:  Int) -> Int  { lhs        % rhs }),
        simple_builtin!(rem (lhs: Real, rhs: Real) -> Real { lhs        % rhs }),
        simple_builtin!(rem (lhs:  Int, rhs: Real) -> Real { lhs as f64 % rhs }),
        simple_builtin!(rem (lhs: Real, rhs:  Int) -> Real { lhs        % rhs as f64 }),
        // Gr
        simple_builtin!(gr (lhs:  Int, rhs:  Int) -> Bool { lhs        > rhs }),
        simple_builtin!(gr (lhs: Real, rhs: Real) -> Bool { lhs        > rhs }),
        simple_builtin!(gr (lhs:  Int, rhs: Real) -> Bool { lhs as f64 > rhs }),
        simple_builtin!(gr (lhs: Real, rhs:  Int) -> Bool { lhs        > rhs as f64 }),
        // Le
        simple_builtin!(le (lhs:  Int, rhs:  Int) -> Bool { lhs        < rhs }),
        simple_builtin!(le (lhs: Real, rhs: Real) -> Bool { lhs        < rhs }),
        simple_builtin!(le (lhs:  Int, rhs: Real) -> Bool { (lhs as f64) < rhs }),
        simple_builtin!(le (lhs: Real, rhs:  Int) -> Bool { lhs        < rhs as f64 }),
        // Leq
        simple_builtin!(leq (lhs:  Int, rhs:  Int) -> Bool { lhs        <= rhs }),
        simple_builtin!(leq (lhs: Real, rhs: Real) -> Bool { lhs        <= rhs }),
        simple_builtin!(leq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 <= rhs }),
        simple_builtin!(leq (lhs: Real, rhs:  Int) -> Bool { lhs        <= rhs as f64 }),
        // Geq
        simple_builtin!(geq (lhs:  Int, rhs:  Int) -> Bool { lhs        >= rhs }),
        simple_builtin!(geq (lhs: Real, rhs: Real) -> Bool { lhs        >= rhs }),
        simple_builtin!(geq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 >= rhs }),
        simple_builtin!(geq (lhs: Real, rhs:  Int) -> Bool { lhs        >= rhs as f64 }),
        // Eq
        simple_builtin!(eq (lhs:  Int, rhs:  Int) -> Bool { lhs        == rhs }),
        simple_builtin!(eq (lhs: Real, rhs: Real) -> Bool { lhs        == rhs }),
        simple_builtin!(eq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 == rhs }),
        simple_builtin!(eq (lhs: Real, rhs:  Int) -> Bool { lhs        == rhs as f64 }),
        // Neq
        simple_builtin!(neq (lhs:  Int, rhs:  Int) -> Bool { lhs        != rhs }),
        simple_builtin!(neq (lhs: Real, rhs: Real) -> Bool { lhs        != rhs }),
        simple_builtin!(neq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 != rhs }),
        simple_builtin!(neq (lhs: Real, rhs:  Int) -> Bool { lhs        != rhs as f64 }),
        // Or
        simple_builtin!(or (lhs: Bool, rhs: Bool) -> Bool { lhs || rhs }),
        // And
        simple_builtin!(or (lhs: Bool, rhs: Bool) -> Bool { lhs && rhs }),
        // Minus
        simple_builtin!(minus (v:  Int) -> Int  { -v }),
        simple_builtin!(minus (v: Real) -> Real { -v }),
        // Not
        simple_builtin!(minus (v: Bool) -> Bool { !v }),
    ])
});
