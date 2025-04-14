use anyhow::{anyhow, Context};
use types::api::{self, Request};

use crate::Client;

impl Client {
    pub(crate) async fn req<REQ: Request>(&self, req: REQ) -> anyhow::Result<REQ::Response> {
        let resp = self
            .client
            .post(self.server_url.join(REQ::ROUTE).unwrap())
            .json(&req)
            .send()
            .await
            .context("reqwest::send failed")?;
        let status = resp.status();
        let text = resp.text().await.context("failed to get response text")?;
        if status.is_success() {
            Ok(serde_json::from_str(&text)
                .with_context(|| format!("failed to parse server response '{text}'"))?)
        } else if status.is_server_error() {
            let err: api::Error = serde_json::from_str(&text)
                .with_context(|| format!("failed to parse server error '{text}'"))?;
            let err: anyhow::Error = err.into();
            Err(err.context("got error from server"))
        } else {
            Err(anyhow!("got unexpected status code: {status}"))
        }
    }
}
