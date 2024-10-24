use types::lang::{FuncCallExpr, Ident, LetExpr, LetExprDefinition};

use crate::lang;

use super::*;

#[test]
fn precedence() {
    assert_eq!(
        lang::expr("1 + 2 * 3"),
        Ok(binary(
            "#add",
            Value::from(1),
            binary("#mul", Value::from(2), Value::from(3))
        )
        .into())
    );

    assert_eq!(
        lang::expr("(1 + 2) * 3"),
        Ok(binary(
            "#mul",
            binary("#add", Value::from(1), Value::from(2)),
            Value::from(3)
        )
        .into())
    );

    assert_eq!(
        lang::expr("x + 1 < y * 2"),
        Ok(binary(
            "#le",
            binary("#add", Ident::from("x"), Value::from(1)),
            binary("#mul", Ident::from("y"), Value::from(2)),
        )
        .into())
    );

    assert_eq!(
        lang::expr("x + 1 < y * 2 | both flag1 flag2"),
        Ok(binary(
            "#or",
            binary(
                "#le",
                binary("#add", Ident::from("x"), Value::from(1)),
                binary("#mul", Ident::from("y"), Value::from(2)),
            ),
            binary("both", Ident::from("flag1"), Ident::from("flag2"))
        )
        .into())
    );
}

#[test]
fn dot_notation() {
    assert_eq!(lang::expr("l.p1.x"), lang::expr("x (p1 l)"));
    assert_eq!(lang::expr("1 + l.p1.x"), lang::expr("1 + x (p1 l)"));
    assert_eq!(lang::expr("1 + l.p1.x + 1"), lang::expr("1 + x (p1 l) + 1"));
}

#[test]
fn _if() {
    // Single case
    assert_eq!(
        lang::if_expr(r#"if is_odd x then "odd""#),
        Ok(IfExpr {
            cases: vec![IfExprCase {
                condition: FuncCallExpr {
                    name: "is_odd".to_string().into(),
                    arguments: vec![Ident::from("x".to_string()).into()]
                }
                .into(),
                value: Value::from("odd".to_string()).into()
            },],
            default_case_value: None
        })
    );

    // Multiple cases
    assert_eq!(
        lang::if_expr(
            r#"if
            is_odd x then "odd"
            is_even x then "even""#
        ),
        Ok(IfExpr {
            cases: vec![
                IfExprCase {
                    condition: FuncCallExpr {
                        name: "is_odd".to_string().into(),
                        arguments: vec![Ident::from("x".to_string()).into()]
                    }
                    .into(),
                    value: Value::from("odd".to_string()).into()
                },
                IfExprCase {
                    condition: FuncCallExpr {
                        name: "is_even".to_string().into(),
                        arguments: vec![Ident::from("x".to_string()).into()]
                    }
                    .into(),
                    value: Value::from("even".to_string()).into()
                },
            ],
            default_case_value: None
        })
    );

    // Multiple cases with default value
    assert_eq!(
        lang::if_expr(
            r#"if
            is_odd x then "odd"
            is_even x then "even"
            else unreachable """#
        ),
        Ok(IfExpr {
            cases: vec![
                IfExprCase {
                    condition: FuncCallExpr {
                        name: "is_odd".to_string().into(),
                        arguments: vec![Ident::from("x".to_string()).into()]
                    }
                    .into(),
                    value: Value::from("odd".to_string()).into()
                },
                IfExprCase {
                    condition: FuncCallExpr {
                        name: "is_even".to_string().into(),
                        arguments: vec![Ident::from("x".to_string()).into()]
                    }
                    .into(),
                    value: Value::from("even".to_string()).into()
                },
            ],
            default_case_value: Some(
                FuncCallExpr {
                    name: "unreachable".to_string().into(),
                    arguments: vec![Value::from("".to_string()).into()]
                }
                .into()
            )
        })
    );
}

#[test]
fn func_call() {
    // Single argument
    assert_eq!(
        lang::func_call_expr("fact 5"),
        Ok(FuncCallExpr {
            name: "fact".into(),
            arguments: vec![Value::from(5).into()]
        })
    );

    // Multiple arguments
    assert_eq!(
        lang::func_call_expr("add 1 2"),
        Ok(FuncCallExpr {
            name: "add".into(),
            arguments: vec![Value::from(1).into(), Value::from(2).into()]
        })
    );

    // Multiple arguments with idents
    assert_eq!(
        lang::func_call_expr("add x y"),
        Ok(FuncCallExpr {
            name: "add".into(),
            arguments: vec![
                Ident::from("x".to_string()).into(),
                Ident::from("y".to_string()).into()
            ]
        })
    );

    // Complex
    assert_eq!(
        lang::func_call_expr("add 1 (sub 2 3)"),
        Ok(FuncCallExpr {
            name: "add".into(),
            arguments: vec![
                Value::from(1).into(),
                FuncCallExpr {
                    name: "sub".into(),
                    arguments: vec![Value::from(2).into(), Value::from(3).into()]
                }
                .into()
            ]
        })
    );
}

#[test]
fn _let() {
    // Single definition
    assert_eq!(
        lang::let_expr("let x = 10 in fact x"),
        Ok(LetExpr {
            definitions: vec![LetExprDefinition {
                name: Ident::from("x".to_string()),
                value_type: None,
                value: Value::from(10).into()
            }],
            value: FuncCallExpr {
                name: Ident::from("fact"),
                arguments: vec![Ident::from("x").into()]
            }
            .into()
        })
    );

    // Multiple definitions
    assert_eq!(
        lang::let_expr("let x = 10, y = 42 in add x y"),
        Ok(LetExpr {
            definitions: vec![
                LetExprDefinition {
                    name: Ident::from("x".to_string()),
                    value_type: None,
                    value: Value::from(10).into()
                },
                LetExprDefinition {
                    name: Ident::from("y".to_string()),
                    value_type: None,
                    value: Value::from(42).into()
                }
            ],
            value: FuncCallExpr {
                name: Ident::from("add"),
                arguments: vec![Ident::from("x").into(), Ident::from("y").into()]
            }
            .into()
        })
    );

    // With type
    assert_eq!(
        lang::let_expr("let x:int = 10, y:int = 42 in add x y"),
        Ok(LetExpr {
            definitions: vec![
                LetExprDefinition {
                    name: Ident::from("x".to_string()),
                    value_type: Some(ValueType::Int),
                    value: Value::from(10).into()
                },
                LetExprDefinition {
                    name: Ident::from("y".to_string()),
                    value_type: Some(ValueType::Int),
                    value: Value::from(42).into()
                }
            ],
            value: FuncCallExpr {
                name: Ident::from("add"),
                arguments: vec![Ident::from("x").into(), Ident::from("y").into()]
            }
            .into()
        })
    );
}
