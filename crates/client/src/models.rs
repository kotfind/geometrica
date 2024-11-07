use std::{
    net::{IpAddr, Ipv4Addr},
    process::Child,
};

use anyhow::Context;
use reqwest::{Client, Url};
use smart_default::SmartDefault;
use types::api::Request;

#[derive(SmartDefault)]
pub struct ConnectionSettings {
    #[default(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub ip: IpAddr,

    #[default(4242)]
    pub port: u16,

    #[default(true)]
    pub do_init_server: bool,

    #[default(true)]
    pub kill_server_on_drop: bool,
    // TODO: server args
}

#[cfg(test)]
impl ConnectionSettings {
    pub fn new_test() -> Self {
        Self {
            port: 0,
            do_init_server: true,
            kill_server_on_drop: true,
            ..Default::default()
        }
    }
}

pub struct Connection {
    pub(crate) server_url: Url,
    pub(crate) client: Client,
    pub(crate) server_process: Option<Child>,
    pub(crate) kill_server_on_drop: bool,
}

#[cfg(test)]
impl Connection {
    pub async fn new_test() -> anyhow::Result<Self> {
        Self::from(ConnectionSettings::new_test()).await
    }
}

impl Connection {
    pub fn kill_server(&mut self) -> anyhow::Result<()> {
        if let Some(child) = &mut self.server_process {
            child.kill().context("failed to kill server process")?;
        }
        Ok(())
    }

    pub async fn req<REQ: Request>(&self, req: REQ) -> anyhow::Result<REQ::Response> {
        let resp = self
            .client
            .post(self.server_url.join(REQ::PATH).unwrap())
            .json(&req)
            .send()
            .await
            .context("reqwest::send failed")?;
        let text = resp.text().await.context("failed to get response text")?;
        let resp = serde_json::from_str(&text)
            .with_context(|| format!("failed to parse json '{text}'"))?;
        // TODO: parse Result, not just value
        Ok(resp)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.kill_server_on_drop {
            self.kill_server().expect("failed to kill server");
        }
    }
}
