use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::{Child, Stdio},
};

use anyhow::{bail, Context};
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
    pub server_process: Option<Child>,
    pub kill_server_on_drop: bool,
}

#[cfg(test)]
impl Connection {
    pub async fn new_test() -> anyhow::Result<Self> {
        Self::from(ConnectionSettings::new_test()).await
    }
}

impl Connection {
    const SCHEMA: &str = "http";

    pub async fn new() -> anyhow::Result<Self> {
        Self::from(Default::default()).await
    }

    pub async fn from(settings: ConnectionSettings) -> anyhow::Result<Self> {
        let client = Client::new();

        let (server_url, child) = Self::get_server_url_and_child(client.clone(), &settings).await?;

        Ok(Connection {
            server_url,
            client,
            server_process: child,
            kill_server_on_drop: settings.kill_server_on_drop,
        })
    }

    async fn get_server_url_and_child(
        client: Client,
        settings: &ConnectionSettings,
    ) -> anyhow::Result<(Url, Option<Child>)> {
        let ConnectionSettings {
            ip,
            port,
            do_init_server,
            ..
        } = settings;

        let server_url = Self::url_from_ip_port(settings.ip, settings.port);

        if Self::can_ping_server(client.clone(), server_url.clone()).await {
            return Ok((server_url, None));
        }

        if !do_init_server || !ip.is_loopback() {
            bail!(
                "failed to connect to server at {server_url}; won't try to spawn as {reason}",
                reason = if !do_init_server {
                    "do_init_server is false"
                } else {
                    "server address is not localhost"
                }
            );
        }

        let (port, child) = Self::spawn_server(*port)
            .await
            .context("failed to spawn server")?;

        Ok((
            Self::url_from_ip_port(Ipv4Addr::LOCALHOST.into(), port),
            Some(child),
        ))
    }

    fn url_from_ip_port(ip: IpAddr, port: u16) -> Url {
        format!("{SCHEMA}://{ip}:{port}", SCHEMA = Self::SCHEMA)
            .parse()
            .unwrap()
    }

    pub fn kill_server(&mut self) -> anyhow::Result<()> {
        if let Some(child) = &mut self.server_process {
            child.kill().context("failed to kill server process")?;
        }
        Ok(())
    }

    async fn spawn_server(port: u16) -> anyhow::Result<(u16, Child)> {
        // TODO: delete tempdir
        let tmpdir = tempfile::tempdir().unwrap();
        let addr_file = tmpdir.path().join("addr");

        let child = if cfg!(debug_assertions) {
            test_bin::get_test_bin("server")
        } else {
            todo!()
        }
        .args([
            "--bind",
            &format!("127.0.0.1:{port}"),
            "--write-addr",
            &addr_file.to_string_lossy(),
        ])
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .context("failed to spawn server process")?;

        // TODO: wait in async
        while !addr_file.exists() { /* BLOCK */ }

        let addr = tokio::fs::read_to_string(addr_file.clone())
            .await
            .with_context(|| {
                format!("failed to read addr_file '{}'", addr_file.to_string_lossy())
            })?;

        let addr: SocketAddr = addr
            .parse()
            .with_context(|| format!("failed to parse {addr} as address"))?;

        assert_eq!(addr.ip(), Ipv4Addr::LOCALHOST);

        Ok((addr.port(), child))
    }

    async fn can_ping_server(client: Client, server_url: Url) -> bool {
        client
            .get(server_url.join("ping").unwrap())
            .send()
            .await
            .and_then(|resp| resp.error_for_status())
            .is_ok()
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn connect() {
        Connection::from(ConnectionSettings {
            do_init_server: true,
            port: 0,
            ..Default::default()
        })
        .await
        .unwrap();
    }
}
