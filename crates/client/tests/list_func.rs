use test_client::TestClient;
use types::{
    core::{Ident, ValueType},
    lang::FunctionSignature,
};

mod test_client;

#[tokio::test]
async fn user_defined() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
            sum x:real y:real -> real = x + y
            avg x:real y:real -> real = (sum x y) / 2.0
        "#,
        )
        .await
        .unwrap();

    let user_defined_funcs = client.list_funcs().await.unwrap().user_defined;

    assert!(user_defined_funcs.len() == 2);

    assert!(user_defined_funcs.contains(&FunctionSignature {
        name: Ident::from("sum"),
        arg_types: vec![ValueType::Real, ValueType::Real]
    }));

    assert!(user_defined_funcs.contains(&FunctionSignature {
        name: Ident::from("avg"),
        arg_types: vec![ValueType::Real, ValueType::Real]
    }));
}

/// Only checks that some of builtins are present.
#[tokio::test]
async fn builtins() {
    let client = TestClient::new().await;
    let builtins = client.list_funcs().await.unwrap().normal_builtins;

    assert!(builtins.contains(&FunctionSignature {
        name: Ident::from("line"),
        arg_types: vec![ValueType::Pt, ValueType::Pt]
    }));

    assert!(builtins.contains(&FunctionSignature {
        name: Ident::from("circ"),
        arg_types: vec![ValueType::Pt, ValueType::Real]
    }));
}

/// Only checks that some of operators are present.
#[tokio::test]
async fn operators() {
    let client = TestClient::new().await;
    let builtins = client.list_funcs().await.unwrap().operators;

    assert!(builtins.contains(&FunctionSignature {
        name: Ident::from("#add"),
        arg_types: vec![ValueType::Pt, ValueType::Pt]
    }));

    assert!(builtins.contains(&FunctionSignature {
        name: Ident::from("#sub"),
        arg_types: vec![ValueType::Real, ValueType::Real]
    }));

    assert!(builtins.contains(&FunctionSignature {
        name: Ident::from("#not"),
        arg_types: vec![ValueType::Bool]
    }));
}
