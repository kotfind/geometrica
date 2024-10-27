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

fn array_eq(lhs: &[Value], rhs: &[Value]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    for (l, r) in lhs.iter().zip(rhs.iter()) {
        if l != r {
            return false;
        }
    }
    true
}

static BUILT_IN_FUNCS: Lazy<HashMap<FunctionSignature, Function>> = Lazy::new(|| {
    // TODO: check for overflow
    HashMap::from([
        // Add
        simple_builtin!(add (lhs:  Int, rhs:  Int) -> Int  { lhs        + rhs }),
        simple_builtin!(add (lhs: Real, rhs: Real) -> Real { lhs        + rhs }),
        simple_builtin!(add (lhs:  Int, rhs: Real) -> Real { lhs as f64 + rhs }),
        simple_builtin!(add (lhs: Real, rhs:  Int) -> Real { lhs        + rhs as f64 }),
        simple_builtin!(add (lhs:  Str, rhs:  Str) -> Str  { lhs + &rhs }),
        simple_builtin!(add (lhs: Array, rhs: Array) -> Array {
            let mut lhs = lhs;
            let mut rhs = rhs;
            lhs.append(&mut rhs);
            lhs
        }),
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
        simple_builtin!(pow (lhs:  Int, rhs:  Int) -> Int  { lhs       .pow(rhs as u32 /* TODO: check if cast fails */) }),
        simple_builtin!(pow (lhs: Real, rhs: Real) -> Real { lhs       .powf(rhs) }),
        simple_builtin!(pow (lhs:  Int, rhs: Real) -> Real { (lhs as f64).powf(rhs) }),
        simple_builtin!(pow (lhs: Real, rhs:  Int) -> Real { lhs       .powi(rhs as i32 /* TODO: check if cast fails */) }),
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
        simple_builtin!(gr (lhs:  Str, rhs:  Str) -> Bool { lhs        > rhs}),
        // Le
        simple_builtin!(le (lhs:  Int, rhs:  Int) -> Bool { lhs        < rhs }),
        simple_builtin!(le (lhs: Real, rhs: Real) -> Bool { lhs        < rhs }),
        simple_builtin!(le (lhs:  Int, rhs: Real) -> Bool { (lhs as f64) < rhs }),
        simple_builtin!(le (lhs: Real, rhs:  Int) -> Bool { lhs        < rhs as f64 }),
        simple_builtin!(le (lhs:  Str, rhs:  Str) -> Bool { lhs        < rhs }),
        // Leq
        simple_builtin!(leq (lhs:  Int, rhs:  Int) -> Bool { lhs        <= rhs }),
        simple_builtin!(leq (lhs: Real, rhs: Real) -> Bool { lhs        <= rhs }),
        simple_builtin!(leq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 <= rhs }),
        simple_builtin!(leq (lhs: Real, rhs:  Int) -> Bool { lhs        <= rhs as f64 }),
        simple_builtin!(leq (lhs:  Str, rhs:  Str) -> Bool { lhs        <= rhs }),
        // Geq
        simple_builtin!(geq (lhs:  Int, rhs:  Int) -> Bool { lhs        >= rhs }),
        simple_builtin!(geq (lhs: Real, rhs: Real) -> Bool { lhs        >= rhs }),
        simple_builtin!(geq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 >= rhs }),
        simple_builtin!(geq (lhs: Real, rhs:  Int) -> Bool { lhs        >= rhs as f64 }),
        simple_builtin!(geq (lhs:  Str, rhs:  Str) -> Bool { lhs        >= rhs }),
        // Eq
        simple_builtin!(eq (lhs:  Int, rhs:  Int) -> Bool { lhs        == rhs }),
        simple_builtin!(eq (lhs: Real, rhs: Real) -> Bool { lhs        == rhs }),
        simple_builtin!(eq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 == rhs }),
        simple_builtin!(eq (lhs: Real, rhs:  Int) -> Bool { lhs        == rhs as f64 }),
        simple_builtin!(eq (lhs:  Str, rhs:  Str) -> Bool { lhs        == rhs }),
        simple_builtin!(eq (lhs: Array, rhs: Array) -> Bool { array_eq(&lhs, &rhs) }),
        // Neq
        simple_builtin!(neq (lhs:  Int, rhs:  Int) -> Bool { lhs        != rhs }),
        simple_builtin!(neq (lhs: Real, rhs: Real) -> Bool { lhs        != rhs }),
        simple_builtin!(neq (lhs:  Int, rhs: Real) -> Bool { lhs as f64 != rhs }),
        simple_builtin!(neq (lhs: Real, rhs:  Int) -> Bool { lhs        != rhs as f64 }),
        simple_builtin!(neq (lhs:  Str, rhs:  Str) -> Bool { lhs        != rhs }),
        simple_builtin!(neq (lhs: Array, rhs: Array) -> Bool { !array_eq(&lhs, &rhs) }),
        // Or
        simple_builtin!(or (lhs: Bool, rhs: Bool) -> Bool { lhs || rhs }),
        // And
        simple_builtin!(and (lhs: Bool, rhs: Bool) -> Bool { lhs && rhs }),
        // Minus
        simple_builtin!(minus (v:  Int) -> Int  { -v }),
        simple_builtin!(minus (v: Real) -> Real { -v }),
        // Not
        simple_builtin!(not (v: Bool) -> Bool { !v }),
        // As
        simple_builtin!(as_bool (v: Bool) -> Bool { v }),
        simple_builtin!(as_bool (v: Int) -> Bool { v != 0 }),
        simple_builtin!(as_bool (v: Real) -> Bool { v != 0.0 }),
        simple_builtin!(as_int (v: Bool) -> Int { if v { 1 } else { 0 } }),
        simple_builtin!(as_int (v: Int) -> Int { v }),
        simple_builtin!(as_int (v: Real) -> Int { v as i64 }),
        // TODO?: simple_builtin!(as_int (v: Str) -> Int { todo!() }),
        simple_builtin!(as_real (v: Bool) -> Real { if v { 1.0 } else { 0.0 } }),
        simple_builtin!(as_real (v: Int) -> Real { v as f64 }),
        simple_builtin!(as_real (v: Real) -> Real { v }),
        simple_builtin!(as_str (v: Bool) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Int) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Real) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Str) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Array) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Point) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Line) -> Str { Value::from(v).to_string() }),
        simple_builtin!(as_str (v: Circle) -> Str { Value::from(v).to_string() }),
        // TODO: none as str
        // TODO: cast str -> *
        // TODO: optimize as_str (don't construct value after just deconstruncting it)
        // Is
        simple_builtin!(is_bool (_v: Bool) -> Bool { true }),
        simple_builtin!(is_bool (_v: Int) -> Bool { false }),
        simple_builtin!(is_bool (_v: Real) -> Bool { false }),
        simple_builtin!(is_bool (_v: Str) -> Bool { false }),
        simple_builtin!(is_bool (_v: Array) -> Bool { false }),
        simple_builtin!(is_bool (_v: Point) -> Bool { false }),
        simple_builtin!(is_bool (_v: Line) -> Bool { false }),
        simple_builtin!(is_bool (_v: Circle) -> Bool { false }),
        simple_builtin!(is_int (_v: Bool) -> Bool { false }),
        simple_builtin!(is_int (_v: Int) -> Bool { true }),
        simple_builtin!(is_int (_v: Real) -> Bool { false }),
        simple_builtin!(is_int (_v: Str) -> Bool { false }),
        simple_builtin!(is_int (_v: Array) -> Bool { false }),
        simple_builtin!(is_int (_v: Point) -> Bool { false }),
        simple_builtin!(is_int (_v: Line) -> Bool { false }),
        simple_builtin!(is_int (_v: Circle) -> Bool { false }),
        simple_builtin!(is_real (_v: Bool) -> Bool { false }),
        simple_builtin!(is_real (_v: Int) -> Bool { false }),
        simple_builtin!(is_real (_v: Real) -> Bool { true }),
        simple_builtin!(is_real (_v: Str) -> Bool { false }),
        simple_builtin!(is_real (_v: Array) -> Bool { false }),
        simple_builtin!(is_real (_v: Point) -> Bool { false }),
        simple_builtin!(is_real (_v: Line) -> Bool { false }),
        simple_builtin!(is_real (_v: Circle) -> Bool { false }),
        simple_builtin!(is_str (_v: Bool) -> Bool { false }),
        simple_builtin!(is_str (_v: Int) -> Bool { false }),
        simple_builtin!(is_str (_v: Real) -> Bool { false }),
        simple_builtin!(is_str (_v: Str) -> Bool { true }),
        simple_builtin!(is_str (_v: Array) -> Bool { false }),
        simple_builtin!(is_str (_v: Point) -> Bool { false }),
        simple_builtin!(is_str (_v: Line) -> Bool { false }),
        simple_builtin!(is_str (_v: Circle) -> Bool { false }),
        simple_builtin!(is_array (_v: Bool) -> Bool { false }),
        simple_builtin!(is_array (_v: Int) -> Bool { false }),
        simple_builtin!(is_array (_v: Real) -> Bool { false }),
        simple_builtin!(is_array (_v: Str) -> Bool { false }),
        simple_builtin!(is_array (_v: Array) -> Bool { true }),
        simple_builtin!(is_array (_v: Point) -> Bool { false }),
        simple_builtin!(is_array (_v: Line) -> Bool { false }),
        simple_builtin!(is_array (_v: Circle) -> Bool { false }),
        simple_builtin!(is_point (_v: Bool) -> Bool { false }),
        simple_builtin!(is_point (_v: Int) -> Bool { false }),
        simple_builtin!(is_point (_v: Real) -> Bool { false }),
        simple_builtin!(is_point (_v: Str) -> Bool { false }),
        simple_builtin!(is_point (_v: Array) -> Bool { false }),
        simple_builtin!(is_point (_v: Point) -> Bool { true }),
        simple_builtin!(is_point (_v: Line) -> Bool { false }),
        simple_builtin!(is_point (_v: Circle) -> Bool { false }),
        simple_builtin!(is_line (_v: Bool) -> Bool { false }),
        simple_builtin!(is_line (_v: Int) -> Bool { false }),
        simple_builtin!(is_line (_v: Real) -> Bool { false }),
        simple_builtin!(is_line (_v: Str) -> Bool { false }),
        simple_builtin!(is_line (_v: Array) -> Bool { false }),
        simple_builtin!(is_line (_v: Point) -> Bool { false }),
        simple_builtin!(is_line (_v: Line) -> Bool { true }),
        simple_builtin!(is_line (_v: Circle) -> Bool { false }),
        simple_builtin!(is_circle (_v: Bool) -> Bool { false }),
        simple_builtin!(is_circle (_v: Int) -> Bool { false }),
        simple_builtin!(is_circle (_v: Real) -> Bool { false }),
        simple_builtin!(is_circle (_v: Str) -> Bool { false }),
        simple_builtin!(is_circle (_v: Array) -> Bool { false }),
        simple_builtin!(is_circle (_v: Point) -> Bool { false }),
        simple_builtin!(is_circle (_v: Line) -> Bool { false }),
        simple_builtin!(is_circle (_v: Circle) -> Bool { true }),
        // optimize is_ construction
    ])
});

#[cfg(test)]
mod test {
    use crate::eval::{Eval, EvalScope};

    use super::*;

    fn eval(expr: &str) -> Value {
        parser::expr(expr).unwrap().eval(&EvalScope::new()).unwrap()
    }

    #[test]
    fn is() {
        // TODO: test point, line, circle
        let value_to_type = [
            ("true", "bool"),
            ("1", "int"),
            ("1.0", "real"),
            (r#""abc""#, "str"),
            ("(1, 2, 3)", "array"),
        ];

        for i in 0..value_to_type.len() {
            for j in 0..=i {
                assert_eq!(
                    eval(&format!(
                        "{value} is {type_}",
                        value = value_to_type[i].0,
                        type_ = value_to_type[j].1
                    )),
                    (i == j).into(),
                );
            }
        }
    }

    #[test]
    fn as_bool() {
        assert_eq!(eval("true as bool"), true.into());
        assert_eq!(eval("5 as bool"), true.into());
        assert_eq!(eval("7.0 as bool"), true.into());

        assert_eq!(eval("false as bool"), false.into());
        assert_eq!(eval("0 as bool"), false.into());
        assert_eq!(eval("0.0 as bool"), false.into());
    }

    #[test]
    fn as_int() {
        assert_eq!(eval("true as int"), 1.into());
        assert_eq!(eval("false as int"), 0.into());
        assert_eq!(eval("5 as int"), 5.into());
        assert_eq!(eval("7.0 as int"), 7.into());
    }

    #[test]
    fn as_real() {
        assert_eq!(eval("true as real"), 1.0.into());
        assert_eq!(eval("false as real"), 0.0.into());
        assert_eq!(eval("5 as real"), 5.0.into());
        assert_eq!(eval("7.0 as real"), 7.0.into());
    }

    #[test]
    fn as_str() {
        assert_eq!(eval("true as str"), "true".to_string().into());
        assert_eq!(eval("false as str"), "false".to_string().into());
        assert_eq!(eval("1 as str"), "1".to_string().into());
        assert_eq!(eval("1.0 as str"), "1.0".to_string().into());
        assert_eq!(
            eval(r#""abacaba" as str"#),
            r#""abacaba""#.to_string().into()
        );
        assert_eq!(eval("(1, 2, 3) as str"), "(1, 2, 3)".to_string().into());
        // TODO: test point, line, circle
    }

    #[test]
    fn add() {
        assert_eq!(eval("1   + 1"), 2.into());
        assert_eq!(eval("1.0 + 1"), 2.0.into());
        assert_eq!(eval("1   + 1.0"), 2.0.into());
        assert_eq!(eval("1.0 + 1.0"), 2.0.into());
        assert_eq!(eval(r#""aba" + "caba""#), "abacaba".to_string().into());
        assert_eq!(
            eval("(1, 2, 3) + (4, 5, 6)"),
            (1..=6).map(Value::from).collect::<Vec<Value>>().into()
        );
    }

    #[test]
    fn sub() {
        assert_eq!(eval("1   - 1"), 0.into());
        assert_eq!(eval("1.0 - 1"), 0.0.into());
        assert_eq!(eval("1   - 1.0"), 0.0.into());
        assert_eq!(eval("1.0 - 1.0"), 0.0.into());
    }

    #[test]
    fn mul() {
        assert_eq!(eval("2   * 2"), 4.into());
        assert_eq!(eval("2.0 * 2"), 4.0.into());
        assert_eq!(eval("2   * 2.0"), 4.0.into());
        assert_eq!(eval("2.0 * 2.0"), 4.0.into());
    }

    #[test]
    fn div() {
        assert_eq!(eval("4   / 2"), 2.into());
        assert_eq!(eval("4.0 / 2"), 2.0.into());
        assert_eq!(eval("4   / 2.0"), 2.0.into());
        assert_eq!(eval("4.0 / 2.0"), 2.0.into());
    }

    #[test]
    fn pow() {
        assert_eq!(eval("2   ^ 2"), 4.into());
        assert_eq!(eval("2.0 ^ 2"), 4.0.into());
        assert_eq!(eval("2   ^ 2.0"), 4.0.into());
        assert_eq!(eval("2.0 ^ 2.0"), 4.0.into());
    }

    #[test]
    fn rem() {
        assert_eq!(eval("9   % 7"), 2.into());
        assert_eq!(eval("9.0 % 7"), 2.0.into());
        assert_eq!(eval("9   % 7.0"), 2.0.into());
        assert_eq!(eval("9.0 % 7.0"), 2.0.into());
    }

    #[test]
    fn gr() {
        assert_eq!(eval("3   > 2"), true.into());
        assert_eq!(eval("3.0 > 2"), true.into());
        assert_eq!(eval("3   > 2.0"), true.into());
        assert_eq!(eval("3.0 > 2.0"), true.into());
        assert_eq!(eval(r#""def" > "abc""#), true.into());

        assert_eq!(eval("3   > 3"), false.into());
        assert_eq!(eval("3.0 > 3"), false.into());
        assert_eq!(eval("3   > 3.0"), false.into());
        assert_eq!(eval("3.0 > 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn le() {
        assert_eq!(eval("2   < 3"), true.into());
        assert_eq!(eval("2.0 < 3"), true.into());
        assert_eq!(eval("2   < 3.0"), true.into());
        assert_eq!(eval("2.0 < 3.0"), true.into());
        assert_eq!(eval(r#""abc" < "def""#), true.into());

        assert_eq!(eval("3   < 3"), false.into());
        assert_eq!(eval("3.0 < 3"), false.into());
        assert_eq!(eval("3   < 3.0"), false.into());
        assert_eq!(eval("3.0 < 3.0"), false.into());
        assert_eq!(eval(r#""abc" > "abc""#), false.into());
    }

    #[test]
    fn geq() {
        assert_eq!(eval("3   >= 2"), true.into());
        assert_eq!(eval("3.0 >= 2"), true.into());
        assert_eq!(eval("3   >= 2.0"), true.into());
        assert_eq!(eval("3.0 >= 2.0"), true.into());
        assert_eq!(eval(r#""def" >= "abc""#), true.into());

        assert_eq!(eval("3   >= 3"), true.into());
        assert_eq!(eval("3.0 >= 3"), true.into());
        assert_eq!(eval("3   >= 3.0"), true.into());
        assert_eq!(eval("3.0 >= 3.0"), true.into());
        assert_eq!(eval(r#""abc" >= "abc""#), true.into());
    }

    #[test]
    fn leq() {
        assert_eq!(eval("2   <= 3"), true.into());
        assert_eq!(eval("2.0 <= 3"), true.into());
        assert_eq!(eval("2   <= 3.0"), true.into());
        assert_eq!(eval("2.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "def""#), true.into());

        assert_eq!(eval("3   <= 3"), true.into());
        assert_eq!(eval("3.0 <= 3"), true.into());
        assert_eq!(eval("3   <= 3.0"), true.into());
        assert_eq!(eval("3.0 <= 3.0"), true.into());
        assert_eq!(eval(r#""abc" <= "abc""#), true.into());
    }

    #[test]
    fn eq() {
        assert_eq!(eval("3   == 3"), true.into());
        assert_eq!(eval("3.0 == 3"), true.into());
        assert_eq!(eval("3   == 3.0"), true.into());
        assert_eq!(eval("3.0 == 3.0"), true.into());
        assert_eq!(eval(r#""abc" == "abc""#), true.into());
        assert_eq!(eval(r#"(1, 2, 3) == (1, 2, 3)"#), true.into());

        assert_eq!(eval("1   == 3"), false.into());
        assert_eq!(eval("1.0 == 3"), false.into());
        assert_eq!(eval("1   == 3.0"), false.into());
        assert_eq!(eval("1.0 == 3.0"), false.into());
        assert_eq!(eval(r#""abc" == "def""#), false.into());
        assert_eq!(eval(r#"(1, 2, 3) == (4, 5, 6)"#), false.into());
    }

    #[test]
    fn neq() {
        assert_eq!(eval("3   != 3"), false.into());
        assert_eq!(eval("3.0 != 3"), false.into());
        assert_eq!(eval("3   != 3.0"), false.into());
        assert_eq!(eval("3.0 != 3.0"), false.into());
        assert_eq!(eval(r#""abc" != "abc""#), false.into());
        assert_eq!(eval(r#"(1, 2, 3) != (1, 2, 3)"#), false.into());

        assert_eq!(eval("1   != 3"), true.into());
        assert_eq!(eval("1.0 != 3"), true.into());
        assert_eq!(eval("1   != 3.0"), true.into());
        assert_eq!(eval("1.0 != 3.0"), true.into());
        assert_eq!(eval(r#""abc" != "def""#), true.into());
        assert_eq!(eval(r#"(1, 2, 3) != (4, 5, 6)"#), true.into());
    }

    #[test]
    fn or() {
        assert_eq!(eval("true | true"), true.into());
        assert_eq!(eval("true | false"), true.into());
        assert_eq!(eval("false | true"), true.into());
        assert_eq!(eval("false | false"), false.into());
    }

    #[test]
    fn and() {
        assert_eq!(eval("true & true"), true.into());
        assert_eq!(eval("true & false"), false.into());
        assert_eq!(eval("false & true"), false.into());
        assert_eq!(eval("false & false"), false.into());
    }

    #[test]
    fn minus() {
        assert_eq!(eval("-1"), (-1).into());
        assert_eq!(eval("-1.0"), (-1.0).into());
    }

    #[test]
    fn not() {
        assert_eq!(eval("!true"), false.into());
        assert_eq!(eval("!false"), true.into());
    }
}
