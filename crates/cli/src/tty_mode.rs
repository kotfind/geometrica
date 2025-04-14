use anyhow::Context;
use client::Client;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

use crate::exec;

/// Note: won't fail if some of commands failed
pub async fn run(client: Client) -> anyhow::Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut writer = BufWriter::new(tokio::io::stdout());
    let mut script: Option<String> = None;
    loop {
        let prompt = if script.is_some() { "_ " } else { "> " };
        writer
            .write_all(prompt.as_bytes())
            .await
            .context("failed to write prompt to stdout")?;
        writer.flush().await.context("failed to flush stdout")?;

        let mut line = String::new();
        if reader
            .read_line(&mut line)
            .await
            .context("failed to read from stdin")?
            == 0
        {
            break;
        }

        let is_delim = line.trim() == ";;";

        match script {
            Some(script_) if is_delim => {
                // ;;
                // ...
                // ;; <- HERE

                let _ = exec(&client, script_).await;

                script = None;
            }
            Some(script_) => {
                // ;;
                // ... <- HERE
                // ;;

                script = Some(script_ + &line);
            }
            None if is_delim => {
                // ;; <- HERE
                // ...
                // ;;

                script = Some("".to_string());
            }
            None => {
                // Not in ;;-block

                let _ = exec(&client, line).await;
            }
        }
    }

    Ok(())
}
