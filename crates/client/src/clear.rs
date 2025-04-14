use anyhow::Context;
use types::api;

use crate::Client;

impl Client {
    pub async fn clear(&self) -> anyhow::Result<()> {
        self.req(api::clear::Request {})
            .await
            .context("failed to clear")?;

        Ok(())
    }
}

// TODO: tests
