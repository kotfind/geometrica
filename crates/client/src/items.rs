use std::collections::HashMap;

use types::{
    api,
    core::{Ident, Value},
};

use crate::Connection;

impl Connection {
    pub async fn get_all_items(&self) -> anyhow::Result<HashMap<Ident, Value>> {
        let resp = self.req(api::items::get_all::Request).await?;

        Ok(resp.items)
    }
}

#[cfg(test)]
mod test {
    // tested in exec.rs
}
