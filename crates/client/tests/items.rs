use test_client::TestClient;
use types::core::Ident;

mod test_client;

#[tokio::test]
async fn get_all() {
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
async fn get_item() {
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

    assert!(client.get_item("x").await.unwrap() == 1.into());
    assert!(client.get_item("y").await.unwrap() == 2.into());
    assert!(client.get_item("z").await.unwrap() == 3.into());
    assert!(client.get_item("t").await.is_err());
}
