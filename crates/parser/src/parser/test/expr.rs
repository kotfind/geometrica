use types::{
    core::Ident,
    lang::{FuncCallExpr, LetExpr, LetExprDefinition},
};

use super::*;

#[test]
fn precedence() {
    assert_eq!(
        lang::expr("1 + 2 * 3"),
        Ok(infix(
            Value::from(1),
            InfixOp::ADD,
            infix(Value::from(2), InfixOp::MUL, Value::from(3))
        )
        .into())
    );

    assert_eq!(
        lang::expr("(1 + 2) * 3"),
        Ok(infix(
            infix(Value::from(1), InfixOp::ADD, Value::from(2)),
            InfixOp::MUL,
            Value::from(3)
        )
        .into())
    );

    assert_eq!(
        lang::expr("x + 1 < y * 2"),
        Ok(infix(
            infix(Ident::from("x"), InfixOp::ADD, Value::from(1)),
            InfixOp::LE,
            infix(Ident::from("y"), InfixOp::MUL, Value::from(2))
        )
        .into())
    );

    assert_eq!(
        lang::expr("x + 1 < y * 2 | both flag1 flag2"),
        Ok(infix(
            infix(
                infix(Ident::from("x"), InfixOp::ADD, Value::from(1)),
                InfixOp::LE,
                infix(Ident::from("y"), InfixOp::MUL, Value::from(2))
            ),
            InfixOp::OR,
            binary_call(
                Ident::from("both"),
                Ident::from("flag1"),
                Ident::from("flag2")
            )
        )
        .into())
    );

    assert_eq!(
        lang::expr("p.x as int"),
        Ok(AsExpr {
            body: Box::new(
                DotExpr {
                    name: Ident::from("x"),
                    body: Box::new(Ident::from("p").into())
                }
                .into()
            ),
            value_type: ValueType::Int
        }
        .into())
    );
}

#[test]
fn type_cast() {
    assert_eq!(
        lang::expr("x as bool"),
        Ok(AsExpr {
            body: Box::new(Ident::from("x").into()),
            value_type: ValueType::Bool
        }
        .into())
    );

    assert_eq!(
        lang::expr("x + 1.0 as int"),
        Ok(infix(
            Ident::from("x"),
            InfixOp::ADD,
            AsExpr {
                body: Box::new(Value::from(1.0).into()),
                value_type: ValueType::Int
            }
        )
        .into())
    );
}

#[test]
fn dot_notation() {
    assert_eq!(
        lang::expr("l.p1.x"),
        Ok(DotExpr {
            name: Ident::from("x"),
            body: Box::new(
                DotExpr {
                    name: Ident::from("p1"),
                    body: Box::new(Ident::from("l").into())
                }
                .into()
            )
        }
        .into())
    );

    assert_eq!(
        lang::expr("1 + l.p1.x * 2"),
        Ok(infix(
            Value::from(1),
            InfixOp::ADD,
            infix(
                DotExpr {
                    name: Ident::from("x"),
                    body: Box::new(
                        DotExpr {
                            name: Ident::from("p1"),
                            body: Box::new(Ident::from("l").into())
                        }
                        .into()
                    )
                },
                InfixOp::MUL,
                Value::from(2)
            )
        )
        .into())
    );
}

#[test]
fn _if() {
    // Single case
    assert_eq!(
        lang::if_expr(r#"if is_odd x then "odd""#),
        Ok(IfExpr {
            cases: vec![IfExprCase {
                cond: Box::new(
                    FuncCallExpr {
                        name: "is_odd".into(),
                        args: vec![Box::new(Ident::from("x").into())]
                    }
                    .into()
                ),
                value: Box::new(Value::from("odd".to_string()).into())
            },],
            default_value: None
        })
    );

    // Multiple cases
    assert_eq!(
        lang::if_expr(
            r#"if
            is_odd x then "odd",
            is_even x then "even""#
        ),
        Ok(IfExpr {
            cases: vec![
                IfExprCase {
                    cond: Box::new(
                        FuncCallExpr {
                            name: "is_odd".into(),
                            args: vec![Box::new(Ident::from("x").into())]
                        }
                        .into()
                    ),
                    value: Box::new(Value::from("odd".to_string()).into())
                },
                IfExprCase {
                    cond: Box::new(
                        FuncCallExpr {
                            name: "is_even".into(),
                            args: vec![Box::new(Ident::from("x").into())]
                        }
                        .into()
                    ),
                    value: Box::new(Value::from("even".to_string()).into())
                },
            ],
            default_value: None
        })
    );

    // Multiple cases with default value
    assert_eq!(
        lang::if_expr(
            r#"if
            is_odd x then "odd",
            is_even x then "even"
            else unreachable """#
        ),
        Ok(IfExpr {
            cases: vec![
                IfExprCase {
                    cond: Box::new(
                        FuncCallExpr {
                            name: "is_odd".into(),
                            args: vec![Box::new(Ident::from("x").into())]
                        }
                        .into()
                    ),
                    value: Box::new(Value::from("odd".to_string()).into())
                },
                IfExprCase {
                    cond: Box::new(
                        FuncCallExpr {
                            name: "is_even".into(),
                            args: vec![Box::new(Ident::from("x").into())]
                        }
                        .into()
                    ),
                    value: Box::new(Value::from("even".to_string()).into())
                },
            ],
            default_value: Some(Box::new(
                FuncCallExpr {
                    name: "unreachable".into(),
                    args: vec![Box::new(Value::from("".to_string()).into())]
                }
                .into()
            ))
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
            args: vec![Box::new(Value::from(5).into())]
        })
    );

    // Multiple arguments
    assert_eq!(
        lang::func_call_expr("add 1 2"),
        Ok(FuncCallExpr {
            name: "add".into(),
            args: vec![
                Box::new(Value::from(1).into()),
                Box::new(Value::from(2).into())
            ]
        })
    );

    // Multiple arguments with idents
    assert_eq!(
        lang::func_call_expr("add x y"),
        Ok(FuncCallExpr {
            name: "add".into(),
            args: vec![
                Box::new(Ident::from("x").into()),
                Box::new(Ident::from("y").into())
            ]
        })
    );

    // Complex
    assert_eq!(
        lang::func_call_expr("add 1 (sub 2 3)"),
        Ok(FuncCallExpr {
            name: "add".into(),
            args: vec![
                Box::new(Value::from(1).into()),
                Box::new(
                    FuncCallExpr {
                        name: "sub".into(),
                        args: vec![
                            Box::new(Value::from(2).into()),
                            Box::new(Value::from(3).into())
                        ]
                    }
                    .into()
                )
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
            defs: vec![LetExprDefinition {
                name: Ident::from("x"),
                value_type: None,
                body: Box::new(Value::from(10).into())
            }],
            body: Box::new(
                FuncCallExpr {
                    name: Ident::from("fact"),
                    args: vec![Box::new(Ident::from("x").into())]
                }
                .into()
            )
        })
    );

    // Multiple definitions
    assert_eq!(
        lang::let_expr("let x = 10, y = 42 in add x y"),
        Ok(LetExpr {
            defs: vec![
                LetExprDefinition {
                    name: Ident::from("x"),
                    value_type: None,
                    body: Box::new(Value::from(10).into())
                },
                LetExprDefinition {
                    name: Ident::from("y"),
                    value_type: None,
                    body: Box::new(Value::from(42).into())
                }
            ],
            body: Box::new(
                FuncCallExpr {
                    name: Ident::from("add"),
                    args: vec![
                        Box::new(Ident::from("x").into()),
                        Box::new(Ident::from("y").into())
                    ]
                }
                .into()
            )
        })
    );

    // With type
    assert_eq!(
        lang::let_expr("let x:int = 10, y:int = 42 in add x y"),
        Ok(LetExpr {
            defs: vec![
                LetExprDefinition {
                    name: Ident::from("x"),
                    value_type: Some(ValueType::Int),
                    body: Box::new(Value::from(10).into())
                },
                LetExprDefinition {
                    name: Ident::from("y"),
                    value_type: Some(ValueType::Int),
                    body: Box::new(Value::from(42).into())
                }
            ],
            body: Box::new(
                FuncCallExpr {
                    name: Ident::from("add"),
                    args: vec![
                        Box::new(Ident::from("x").into()),
                        Box::new(Ident::from("y").into())
                    ]
                }
                .into()
            )
        })
    );
}

#[test]
fn array() {
    assert_eq!(
        lang::expr("(1, 2, 3)"),
        Ok(Value::from(vec![1.into(), 2.into(), 3.into()]).into())
    );
}
