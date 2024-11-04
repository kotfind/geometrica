mod connection;
use connection::Connection;
use types::{api, core::Ident};

#[tokio::test]
async fn simple() {
    let con = Connection::new();
    let resp: api::eval::Response = con
        .json_req(
            "/eval",
            &api::eval::Request {
                exprs: vec![
                    api::eval::RequestExpr {
                        expr: "1 + 1".to_string(),
                        vars: [].into(),
                    },
                    api::eval::RequestExpr {
                        expr: "(2 + 1) * 3".to_string(),
                        vars: [].into(),
                    },
                ],
            },
        )
        .await;
    assert_eq!(
        resp,
        api::eval::Response {
            values: vec![Ok(2.into()), Ok(9.into())]
        }
    )
}

#[tokio::test]
async fn with_vars() {
    let con = Connection::new();
    let resp: api::eval::Response = con
        .json_req(
            "/eval",
            &api::eval::Request {
                exprs: vec![api::eval::RequestExpr {
                    expr: "x + y".to_string(),
                    vars: [(Ident::from("x"), 10.into()), (Ident::from("y"), 15.into())].into(),
                }],
            },
        )
        .await;
    assert_eq!(
        resp,
        api::eval::Response {
            values: vec![Ok(25.into())]
        }
    )
}
