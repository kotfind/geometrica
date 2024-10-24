use super::*;

#[test]
fn script() {
    assert_eq!(
        lang::script(
            r#"
            p = point 1 2;
            move p 3 4
        "#
        ),
        Ok(vec![
            Definition::ValueDefinition(ValueDefinition {
                name: Ident::from("p"),
                value_type: None,
                body: binary("point", Value::from(1), Value::from(2)).into(),
            })
            .into(),
            Command {
                name: Ident::from("move"),
                arguments: vec![
                    Ident::from("p").into(),
                    Value::from(3).into(),
                    Value::from(4).into(),
                ],
            }
            .into(),
        ])
    );
}

#[test]
fn statement() {
    assert_eq!(
        lang::statement("p = point 1 2"),
        Ok(Definition::ValueDefinition(ValueDefinition {
            name: Ident::from("p"),
            value_type: None,
            body: binary("point", Value::from(1), Value::from(2)).into()
        })
        .into())
    );

    assert_eq!(
        lang::statement("move p 3 4"),
        Ok(Command {
            name: Ident::from("move"),
            arguments: vec![
                Ident::from("p").into(),
                Value::from(3).into(),
                Value::from(4).into()
            ]
        }
        .into())
    );
}

#[test]
fn command() {
    assert_eq!(
        lang::command("move l 1 2"),
        Ok(Command {
            name: Ident::from("move"),
            arguments: vec![
                Ident::from("l").into(),
                Value::from(1).into(),
                Value::from(2).into()
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
            arguments: vec![
                FunctionDefinitionArgument {
                    name: Ident::from("x"),
                    value_type: ValueType::Int.into()
                },
                FunctionDefinitionArgument {
                    name: Ident::from("y"),
                    value_type: ValueType::Int.into()
                },
            ],
            return_type: ValueType::Int,
            body: binary(
                Ident::from("#add"),
                binary(Ident::from("#pow"), Ident::from("x"), Value::from(2)),
                binary(Ident::from("#pow"), Ident::from("y"), Value::from(2))
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
