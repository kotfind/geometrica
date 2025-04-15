use test_client::TestClient;

mod test_client;

#[tokio::test]
async fn set() {
    TestClient::new().await;
}
