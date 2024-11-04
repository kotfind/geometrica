mod connection;
use connection::Connection;
use types::{api, core::Ident};

#[tokio::test]
async fn simple() {
    let con = Connection::new();

    con.json_req::<api::exec::Response>(
        "/exec",
        &api::exec::Request {
            script: r#"
                x = 1;
                y = 2;
                z = x + y;
            "#
            .to_string(),
        },
    )
    .await;

    let resp = con
        .json_req::<api::items::get_all::Response>("/items/get_all", &api::items::get_all::Request)
        .await;

    assert_eq!(
        resp,
        api::items::get_all::Response {
            items: [
                (Ident::from("x"), 1.into()),
                (Ident::from("y"), 2.into()),
                (Ident::from("z"), 3.into()),
            ]
            .into()
        }
    );
}

#[tokio::test]
async fn with_funcs() {
    let con = Connection::new();

    con.json_req::<api::exec::Response>(
        "/exec",
        &api::exec::Request {
            script: r#"
                sq x:int -> int = x^2;
                sq x:real -> real = x^2;
                sum x:int y:int -> int = x + y;
                a = 1;
                b = 2;
                c = sum (sq a) (sq b);
            "#
            .to_string(),
        },
    )
    .await;

    let resp = con
        .json_req::<api::items::get_all::Response>("/items/get_all", &api::items::get_all::Request)
        .await;

    assert_eq!(
        resp,
        api::items::get_all::Response {
            items: [
                (Ident::from("a"), 1.into()),
                (Ident::from("b"), 2.into()),
                (Ident::from("c"), 5.into()),
            ]
            .into()
        }
    );
}

#[tokio::test]
async fn multiple_requests() {
    let con = Connection::new();

    con.json_req::<api::exec::Response>(
        "/exec",
        &api::exec::Request {
            script: r#"
                sq x:int -> int = x^2;
                sq x:real -> real = x^2;
                sum x:int y:int -> int = x + y;
            "#
            .to_string(),
        },
    )
    .await;

    con.json_req::<api::exec::Response>(
        "/exec",
        &api::exec::Request {
            script: r#"
                a = 1;
                b = 2;
                c = sum (sq a) (sq b);
            "#
            .to_string(),
        },
    )
    .await;

    let resp = con
        .json_req::<api::items::get_all::Response>("/items/get_all", &api::items::get_all::Request)
        .await;

    assert_eq!(
        resp,
        api::items::get_all::Response {
            items: [
                (Ident::from("a"), 1.into()),
                (Ident::from("b"), 2.into()),
                (Ident::from("c"), 5.into()),
            ]
            .into()
        }
    );
}
