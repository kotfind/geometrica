use test_client::TestClient;

mod test_client;

#[tokio::test]
async fn eval() {
    let con = TestClient::new().await;
    assert_eq!(con.eval_one("1 + 1").await.unwrap(), 2.into());
}

#[tokio::test]
async fn eval_multi() {
    let con = TestClient::new().await;
    let mut res = con
        .eval(["1 + 1", "2 * 2", "x + 1"])
        .await
        .unwrap()
        .into_iter();

    assert_eq!(res.next().unwrap().unwrap(), 2.into());
    assert_eq!(res.next().unwrap().unwrap(), 4.into());
    assert!(res.next().unwrap().is_err());
    assert!(res.next().is_none());
}
