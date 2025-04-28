use test_client::TestClient;

mod test_client;

#[tokio::test]
async fn eval() {
    let client = TestClient::new().await;
    assert_eq!(client.eval_one("1 + 1").await.unwrap(), 2.into());
}

#[tokio::test]
async fn eval_multi() {
    let client = TestClient::new().await;
    let mut res = client
        .eval(["1 + 1", "2 * 2", "x + 1"])
        .await
        .unwrap()
        .into_iter();

    assert_eq!(res.next().unwrap().unwrap(), 2.into());
    assert_eq!(res.next().unwrap().unwrap(), 4.into());
    assert!(res.next().unwrap().is_err());
    assert!(res.next().is_none());
}
