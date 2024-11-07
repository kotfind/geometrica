use anyhow::Context;
use client::Connection;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run(con: Connection) -> anyhow::Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut line = String::new();
        if reader
            .read_line(&mut line)
            .await
            .context("stdin read failed")?
            == 0
        {
            break;
        }

        con.exec(line).await.context("failed to execute line")?;
    }

    Ok(())
}
