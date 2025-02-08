use crate::Client;
use parser::ParseInto;
use types::{api, core::Ident, lang::Expr};

impl Client {
    pub async fn set(
        &self,
        name: impl Into<Ident>,
        expr: impl ParseInto<Expr>,
    ) -> anyhow::Result<()> {
        self.req(api::set::Request {
            name: name.into(),
            expr: expr.parse_into()?,
        })
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::TestClient;

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
}
