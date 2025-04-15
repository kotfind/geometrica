use test_client::TestClient;
use types::core::Ident;

mod test_client;

#[tokio::test]
async fn simple() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
            x = 1
            y = 2
            z = x + y
        "#,
        )
        .await
        .unwrap();

    let items = client.get_all_items().await.unwrap();

    assert!(items.len() == 3);
    assert!(items[&Ident::from("x")] == 1.into());
    assert!(items[&Ident::from("y")] == 2.into());
    assert!(items[&Ident::from("z")] == 3.into());
}

#[tokio::test]
async fn with_funcs() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
                sq x:int -> int = x^2
                sq x:real -> real = x^2.0
                sum x:int y:int -> int = x + y
                a = 1
                b = 2
                c = sum (sq a) (sq b)
        "#,
        )
        .await
        .unwrap();

    let items = client.get_all_items().await.unwrap();

    assert!(items.len() == 3);
    assert!(items[&Ident::from("a")] == 1.into());
    assert!(items[&Ident::from("b")] == 2.into());
    assert!(items[&Ident::from("c")] == 5.into());
}

#[tokio::test]
async fn multiple_requests() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
            sq x:int -> int = x^2
            sq x:real -> real = x^2.0
            sum x:int y:int -> int = x + y
        "#,
        )
        .await
        .unwrap();

    client
        .define(
            r#"
            a = 1
            b = 2
            c = sum (sq a) (sq b)
        "#,
        )
        .await
        .unwrap();

    let items = client.get_all_items().await.unwrap();

    assert!(items.len() == 3);
    assert!(items[&Ident::from("a")] == 1.into());
    assert!(items[&Ident::from("b")] == 2.into());
    assert!(items[&Ident::from("c")] == 5.into());
}
