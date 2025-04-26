use test_client::TestClient;

mod test_client;

#[tokio::test]
async fn clear() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
            x = 1
            y = 2
            sum x:int y:int -> int = x + y
            "#,
        )
        .await
        .unwrap();

    client.clear().await.unwrap();

    let items = client.get_all_items().await.unwrap();
    assert!(items.is_empty());

    let user_defined_funcs = client.list_funcs().await.unwrap().user_defined;
    assert!(user_defined_funcs.is_empty());
}
