use types::{api, core::Ident};

use crate::Client;

impl Client {
    pub async fn rm(&self, name: impl Into<Ident>) -> anyhow::Result<()> {
        self.req(api::rm::Request { name: name.into() }).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils::TestClient;

    #[tokio::test]
    async fn rm() {
        let client = TestClient::new().await;

        client
            .define(
                r#"
                x = 1.0
                y = 2.0 * x
                z = 3.0
                p1 = pt x y
                p2 = pt 1.0 z
            "#,
            )
            .await
            .unwrap();

        client.rm("x").await.unwrap();

        let items = client.get_all_items().await.unwrap();

        assert!(items.len() == 2);
        assert!(items.contains_key(&Ident::from("z")));
        assert!(items.contains_key(&Ident::from("p2")));
    }
}
