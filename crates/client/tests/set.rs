use test_client::TestClient;

mod test_client;

#[tokio::test]
async fn set() {
    let client = TestClient::new().await;

    client
        .define(
            r#"
        x = 2
        y = 3
        z = x^2 + y^2
    "#,
        )
        .await
        .unwrap();

    assert_eq!(client.get_item("z").await.unwrap(), 13.into());

    client.set("x", "5").await.unwrap();
    client.set("y", "7").await.unwrap();

    assert_eq!(client.get_item("z").await.unwrap(), 74.into());
}
