use std::collections::HashMap;

use types::{
    api,
    core::{Ident, Value},
};

use crate::Client;

impl Client {
    pub async fn get_all_items(&self) -> anyhow::Result<HashMap<Ident, Value>> {
        let resp = self.req(api::items::get_all::Request).await?;

        Ok(resp.items)
    }
}

impl Client {
    pub async fn get_item(&self, name: impl Into<Ident>) -> anyhow::Result<Value> {
        let resp = self
            .req(api::items::get::Request { name: name.into() })
            .await?;

        Ok(resp.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_all() {
        let con = Client::new_test().await.unwrap();

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
    async fn get_item() {
        let con = Client::new_test().await.unwrap();

        con.define(
            r#"
            x = 1
            y = 2
            z = x + y
        "#,
        )
        .await
        .unwrap();

        assert!(con.get_item("x").await.unwrap() == 1.into());
        assert!(con.get_item("y").await.unwrap() == 2.into());
        assert!(con.get_item("z").await.unwrap() == 3.into());
        assert!(con.get_item("t").await.is_err());
    }
}
