use std::{ops::Deref, process::Child};

use reqwest::Url;

use crate::{Client, ClientSettings};

pub struct TestClient {
    client: Client,
    child: Child,
}

impl TestClient {
    pub async fn new() -> Self {
        let (client, child) = Client::from_with_child(ClientSettings {
            server_url: Url::parse("http://127.0.0.1:0").unwrap(),
            try_spawn_server: true,
        })
        .await
        .expect("failed to spawn server");

        TestClient {
            client,
            child: child.expect("server should have been spawned"),
        }
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        self.child.kill().expect("failed to kill server process");
    }
}

impl Deref for TestClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
