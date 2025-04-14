use crate::Client;
use anyhow::Context;
use parser::ParseInto;
use types::{api, core::Ident, lang::Expr};

impl Client {
    pub async fn set(
        &self,
        name: impl Into<Ident>,
        expr: impl ParseInto<Expr>,
    ) -> anyhow::Result<()> {
        let name = name.into();
        let expr = expr.parse_into().context("failed to parse expr")?;
        self.req(api::set::Request {
            name: name.clone(),
            expr: expr.clone(),
        })
        .await
        .context(format!("failed to set '{name}' to '{expr}'"))?;

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
