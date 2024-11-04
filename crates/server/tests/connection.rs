use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::Read,
    process::{Child, Stdio},
};

pub struct Connection {
    child: Child,
    url: String,
    client: Client,
}

impl Connection {
    pub fn new() -> Self {
        // init server
        let mut child = test_bin::get_test_bin("server")
            .stderr(Stdio::piped())
            .args(["--bind", "127.0.0.1:0", "--print-addr"])
            .spawn()
            .unwrap();

        let stderr = child.stderr.as_mut().unwrap();
        let mut line = String::new();
        let mut c = [0u8];
        loop {
            stderr.read_exact(&mut c).unwrap();
            let ch = c[0] as char;
            if ch == '\n' {
                break;
            }
            line.push(ch);
        }

        let line_parts: Vec<_> = line.split(':').collect();
        let ip = line_parts[1];
        let port: u16 = line_parts[2].parse().unwrap();

        assert_eq!(line_parts[0], "listener_addr");
        assert_eq!(ip, "127.0.0.1");

        let url = format!("http://{ip}:{port}");

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
