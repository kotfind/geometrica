use super::*;

#[test]
fn script() {
    assert_eq!(
        lang::script(
            r#"
            p = pt 1 2
            move! p (1 + 3) 4
        "#
        ),
        Ok(vec![
            Definition::ValueDefinition(ValueDefinition {
                name: Ident::from("p"),
                value_type: None,
                body: binary_call("pt", Value::from(1), Value::from(2)).into(),
            })
            .into(),
            Command {
                name: Ident::from("move"),
                args: vec![
                    Ident::from("p").into(),
                    lang::expr("1 + 3").unwrap().into(),
                    Expr::from(Value::from(4)).into(),
                ],
            }
            .into(),
        ])
    );
}

#[test]
fn statement() {
    assert_eq!(
        lang::statement("p = pt 1 2"),
        Ok(Definition::ValueDefinition(ValueDefinition {
            name: Ident::from("p"),
            value_type: None,
            body: binary_call("pt", Value::from(1), Value::from(2)).into()
        })
        .into())
    );

    assert_eq!(
        lang::statement("move! p 3 4"),
        Ok(Command {
            name: Ident::from("move"),
            args: vec![
                Ident::from("p").into(),
                Expr::from(Value::from(3)).into(),
                Expr::from(Value::from(4)).into()
            ]
        }
        .into())
    );
}

#[test]
fn command() {
    assert_eq!(
        lang::command("move! l 1 2"),
        Ok(Command {
            name: Ident::from("move"),
            args: vec![
                Ident::from("l").into(),
                Expr::from(Value::from(1)).into(),
                Expr::from(Value::from(2)).into()
            ]
        })
    );
}

#[test]
fn function_definition() {
    assert_eq!(
        lang::function_definition("hypot x:int y:int -> int = x^2 + y^2"),
        Ok(FunctionDefinition {
            name: Ident::from("hypot"),
            args: vec![
                FunctionDefinitionArgument {
                    name: Ident::from("x"),
                    value_type: ValueType::Int
                },
                FunctionDefinitionArgument {
                    name: Ident::from("y"),
                    value_type: ValueType::Int
                },
            ],
            return_type: ValueType::Int,
            body: infix(
                infix(Ident::from("x"), InfixOp::POW, Value::from(2)),
                InfixOp::ADD,
                infix(Ident::from("y"), InfixOp::POW, Value::from(2)),
            )
            .into()
        })
    );
}

#[test]
fn value_definition() {
    // With type
    assert_eq!(
        lang::value_definition("x:int = 10"),
        Ok(ValueDefinition {
            name: Ident::from("x"),
            value_type: Some(ValueType::Int),
            body: Value::from(10).into()
        })
    );

    // Without type
    assert_eq!(
        lang::value_definition("x = 10"),
        Ok(ValueDefinition {
            name: Ident::from("x"),
            value_type: None,
            body: Value::from(10).into()
        })
    );

    // With complex expr
    assert_eq!(
        lang::value_definition("x:int = 1 + 7.fact"),
        Ok(ValueDefinition {
            name: Ident::from("x"),
            value_type: Some(ValueType::Int),
            body: lang::expr("1 + 7.fact").unwrap()
        })
    );
}
