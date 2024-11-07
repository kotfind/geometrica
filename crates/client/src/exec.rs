use types::api;

use crate::Connection;

impl Connection {
    pub async fn exec(&self, script: impl ToString) -> anyhow::Result<()> {
        self.req(api::exec::Request {
            script: script.to_string(),
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use types::core::Ident;

    use super::*;

    #[tokio::test]
    async fn simple() {
        let con = Connection::new_test().await.unwrap();

        con.exec(
            r#"
            x = 1;
            y = 2;
            z = x + y;
        "#,
        )
        .await
        .unwrap();

        let items = con.get_all_items().await.unwrap();

        assert!(items.len() == 3);
        assert!(items[&Ident::from("x")] == 1.into());
        assert!(items[&Ident::from("y")] == 2.into());
        assert!(items[&Ident::from("z")] == 3.into());
    }

    #[tokio::test]
    async fn with_funcs() {
        let con = Connection::new_test().await.unwrap();

        con.exec(
            r#"
                sq x:int -> int = x^2;
                sq x:real -> real = x^2;
                sum x:int y:int -> int = x + y;
                a = 1;
                b = 2;
                c = sum (sq a) (sq b);
        "#,
        )
        .await
        .unwrap();

        let items = con.get_all_items().await.unwrap();

        assert!(items.len() == 3);
        assert!(items[&Ident::from("a")] == 1.into());
        assert!(items[&Ident::from("b")] == 2.into());
        assert!(items[&Ident::from("c")] == 5.into());
    }

    #[tokio::test]
    async fn multiple_requests() {
        let con = Connection::new_test().await.unwrap();

        con.exec(
            r#"
            sq x:int -> int = x^2;
            sq x:real -> real = x^2;
            sum x:int y:int -> int = x + y;
        "#,
        )
        .await
        .unwrap();

        con.exec(
            r#"
            a = 1;
            b = 2;
            c = sum (sq a) (sq b);
        "#,
        )
        .await
        .unwrap();

        let items = con.get_all_items().await.unwrap();

        assert!(items.len() == 3);
        assert!(items[&Ident::from("a")] == 1.into());
        assert!(items[&Ident::from("b")] == 2.into());
        assert!(items[&Ident::from("c")] == 5.into());
    }
}
