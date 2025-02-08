use anyhow::{anyhow, Context};
use reqwest::Url;
use smart_default::SmartDefault;
use types::api::{self, Request};

#[derive(SmartDefault)]
pub struct ClientSettings {
    #[default(Url::parse(ClientSettings::DEFAULT_URL).unwrap())]
    pub server_url: Url,

    /// Try to spawn a server if connection failed and server_url is a loopback ip.
    #[default(true)]
    pub try_spawn_server: bool,
    // TODO: server args
}

impl ClientSettings {
    /// Default url for server. Can be safely parsed to [Url].
    pub const DEFAULT_URL: &str = "http://127.0.0.1:4242";
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) server_url: Url,
    pub(crate) client: reqwest::Client,
}

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
