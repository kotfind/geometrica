use anyhow::Context;
use client::Client;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::print_all_items;

pub async fn run(con: Client) -> anyhow::Result<()> {
    let mut script = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    while reader
        .read_line(&mut script)
        .await
        .context("stdin read failed")?
        != 0
    {}

    con.define(script)
        .await
        .context("failed to execute script")?;

    print_all_items(con).await?;

    Ok(())
}
