use std::path::PathBuf;

use anyhow::Context;
use client::Connection;

use crate::print_all_items;

pub async fn run(con: Connection, script_file: PathBuf) -> anyhow::Result<()> {
    let script = tokio::fs::read_to_string(script_file)
        .await
        .context("failed to read script file")?;

    con.define(script)
        .await
        .context("failed to execute script")?;

    print_all_items(con).await?;

    Ok(())
}
