use std::path::PathBuf;

use anyhow::Context;
use client::Client;

use crate::exec;

pub async fn run(client: Client, script_file: PathBuf) -> anyhow::Result<()> {
    let script = tokio::fs::read_to_string(script_file)
        .await
        .context("failed to read script file")?;

    exec(&client, script).await?;

    Ok(())
}
