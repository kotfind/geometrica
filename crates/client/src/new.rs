use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::{Child, Stdio},
};

use anyhow::{bail, Context};
use reqwest::{Client, Url};

use crate::{models::ConnectionSettings, Connection};

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
            .post(server_url.join("ping").unwrap())
            .send()
            .await
            .and_then(|resp| resp.error_for_status())
            .is_ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn connect() {
        Connection::new_test().await.unwrap();
    }
}
