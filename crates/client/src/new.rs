use std::process::{Child, Command, Stdio};

use anyhow::{bail, Context};
use reqwest::Url;
use smart_default::SmartDefault;
use url::Host;

use crate::Client;

static SERVER_BINARY_NAME: &str = "server";

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

impl Client {
    const SCHEMA: &str = "http";

    pub async fn new() -> anyhow::Result<Self> {
        Self::from(Default::default()).await
    }

    /// Same as [Self::from_with_child] but won't return [Child].
    pub async fn from(settings: ClientSettings) -> anyhow::Result<Self> {
        Self::from_with_child(settings)
            .await
            .map(|(client, _child)| client)
    }

    /// Tries to create a client, and returns the client and optionaly server's process.
    /// Process provided if and only if server was spawned by this client.
    pub async fn from_with_child(
        settings: ClientSettings,
    ) -> anyhow::Result<(Self, Option<Child>)> {
        let client = reqwest::Client::new();
        let server_url = settings.server_url;

        if server_url.fragment().is_some() {
            bail!("server_url shouldn't have a fragment");
        }

        if Self::ping_server_with_url(client.clone(), server_url.clone()).await {
            return Ok((Client { server_url, client }, None));
        }

        let server_to_spawn_port =
            Self::is_spawnable(server_url.clone(), settings.try_spawn_server)
                .context("won's try spawning a server")
                .context("failed to connect to server")?;

        let (server_port, server_child) = Self::spawn_server(server_to_spawn_port)
            .await
            .context("failed to spawn server")?;

        let server_url = Url::parse(&format!("http://127.0.0.1:{server_port}"))
            .expect("failed to parse server url");

        Ok((Client { server_url, client }, Some(server_child)))
    }

    /// Return Err if NOT spawnable and a port for server to listen on otherwise.
    fn is_spawnable(server_url: Url, try_spawn_server: bool) -> anyhow::Result<u16> {
        if !try_spawn_server {
            bail!("settings.try_spawn_server = false");
        }

        if server_url.scheme() != Self::SCHEMA {
            bail!("unsupported schema in server_url");
        }

        {
            let path = server_url.path();
            if path != "/" && !path.is_empty() {
                bail!("server_url path is not empty: '{path}'");
            }
        }

        match server_url.host() {
            None => {
                bail!("host not provided");
            }
            Some(Host::Domain("localhost")) => {}
            Some(Host::Ipv4(ip)) if ip.is_loopback() => {}
            Some(Host::Ipv6(ip)) if ip.is_loopback() => {}
            _ => {
                bail!("host is not a loopback");
            }
        }

        Ok(server_url.port().unwrap_or(4242))
    }

    /// Spawns a new server and returns a number of server's port (useful if port argument was 0)
    /// and server's process.
    async fn spawn_server(port: u16) -> anyhow::Result<(u16, Child)> {
        // TODO: delete tempdir
        let tmpdir = tempfile::tempdir().unwrap();
        let port_file = tmpdir.path().join("port");

        let mut child = Command::new(SERVER_BINARY_NAME)
            .args([
                "--bind",
                &format!("127.0.0.1:{port}"),
                "--port-file",
                &port_file.to_string_lossy(),
            ])
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .context("failed to spawn server process")?;

        // TODO: wait in async
        while !port_file.exists() {
            // BLOCK
            if let Some(code) = child.try_wait().context("failed to read server status")? {
                bail!("server failed with code {code}");
            }
        }

        let port = tokio::fs::read_to_string(port_file.clone())
            .await
            .with_context(|| {
                format!("failed to read port_file '{}'", port_file.to_string_lossy())
            })?;

        let port: u16 = port
            .parse()
            .with_context(|| format!("failed to parse {port} as port"))?;

        Ok((port, child))
    }

    async fn ping_server_with_url(client: reqwest::Client, server_url: Url) -> bool {
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
    use crate::test_utils::TestClient;

    #[tokio::test]
    async fn connect() {
        TestClient::new().await;
    }
}
