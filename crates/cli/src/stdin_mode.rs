use anyhow::Context;
use client::Client;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::exec;

pub async fn run(client: Client) -> anyhow::Result<()> {
    let mut script = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    while reader
        .read_line(&mut script)
        .await
        .context("stdin read failed")?
        != 0
    {}

    exec(&client, script).await?;

    Ok(())
}
