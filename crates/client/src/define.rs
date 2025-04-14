use anyhow::Context;
use parser::ParseInto;
use types::{api, lang::Definition};

use crate::Client;

impl Client {
    pub async fn define_one(&self, def: impl ParseInto<Definition>) -> anyhow::Result<()> {
        self.req(api::exec::Request {
            defs: vec![def.parse_into().context("failed to parse definition")?],
        })
        .await
        .context("define failed")?;

        Ok(())
    }

    pub async fn define(&self, defs: impl ParseInto<Vec<Definition>>) -> anyhow::Result<()> {
        self.req(api::exec::Request {
            defs: defs.parse_into().context("failed to parse definitions")?,
        })
        .await
        .context("define failed")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use types::core::Ident;

    use crate::test_utils::TestClient;

    #[tokio::test]
    async fn simple() {
        let con = TestClient::new().await;

        con.define(
            r#"
            x = 1
            y = 2
            z = x + y
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
        let con = TestClient::new().await;

        con.define(
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

        let items = con.get_all_items().await.unwrap();

        assert!(items.len() == 3);
        assert!(items[&Ident::from("a")] == 1.into());
        assert!(items[&Ident::from("b")] == 2.into());
        assert!(items[&Ident::from("c")] == 5.into());
    }

    #[tokio::test]
    async fn multiple_requests() {
        let con = TestClient::new().await;

        con.define(
            r#"
            sq x:int -> int = x^2
            sq x:real -> real = x^2.0
            sum x:int y:int -> int = x + y
        "#,
        )
        .await
        .unwrap();

        con.define(
            r#"
            a = 1
            b = 2
            c = sum (sq a) (sq b)
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
