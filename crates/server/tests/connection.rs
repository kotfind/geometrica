use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::{net::SocketAddr, process::Child};

pub struct Connection {
    child: Child,
    url: String,
    client: Client,
}

impl Connection {
    pub fn new() -> Self {
        let tmpdir = tempfile::tempdir().unwrap();
        let addr_file = tmpdir.path().join("addr");

        // init server
        let child = test_bin::get_test_bin("server")
            .args([
                "--bind",
                "127.0.0.1:0",
                "--write-addr",
                &addr_file.to_string_lossy(),
            ])
            .spawn()
            .unwrap();

        while !addr_file.exists() { /* BLOCK */ }

        let addr: SocketAddr = std::fs::read_to_string(addr_file).unwrap().parse().unwrap();

        let url = format!("http://{}:{}", addr.ip(), addr.port());

        // init client
        let client = Client::new();

        Self { child, url, client }
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn route(&self, route: &str) -> String {
        let url = self.url();
        let sep = if route.is_empty() || route.starts_with("/") {
            ""
        } else {
            "/"
        };
        format!("{url}{sep}{route}")
    }

    pub async fn json_req<RESP: DeserializeOwned>(
        &self,
        route: &str,
        body: &impl Serialize,
    ) -> RESP {
        let resp = self
            .client
            .post(self.route(route))
            .json(body)
            .send()
            .await
            .expect("sending a request failed")
            .error_for_status()
            .expect("got bad status code");
        let text = resp.text().await.unwrap();
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("failed to parse '{text}': {e}"))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.child.kill().unwrap();
    }
}
