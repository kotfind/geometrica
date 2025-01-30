use std::{
    net::{IpAddr, Ipv4Addr},
    process::Child,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context};
use reqwest::Url;
use smart_default::SmartDefault;
use types::api::{self, Request};

#[derive(SmartDefault)]
pub struct ClientSettings {
    #[default(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub ip: IpAddr,

    #[default(4242)]
    pub port: u16,

    #[default(true)]
    pub do_init_server: bool,

    #[default(false)]
    pub kill_server_on_drop: bool,
    // TODO: server args
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) server_url: Url,
    pub(crate) client: reqwest::Client,
    // FIXME
    // Arc<Mutex<...>> is a workarround to make client Clone-able
    pub(crate) server_process: Arc<Mutex<Option<Child>>>,
    pub(crate) kill_server_on_drop: bool,
}

impl Client {
    pub(crate) fn kill_server(&mut self) -> anyhow::Result<()> {
        if let Some(child) = &mut *self.server_process.lock().unwrap() {
            child.kill().context("failed to kill server process")?;
        }
        Ok(())
    }

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

impl Drop for Client {
    fn drop(&mut self) {
        if self.kill_server_on_drop {
            self.kill_server().expect("failed to kill server");
        }
    }
}
