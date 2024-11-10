use anyhow::Context;
use client::Client;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::exec;

pub async fn run(client: Client) -> anyhow::Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut script: Option<String> = None;
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

        let is_delim = line.trim() == ";;";

        match script {
            Some(script_) if is_delim => {
                // ;;
                // ...
                // ;; <- HERE

                exec(&client, script_).await?;

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

                exec(&client, line).await?;
            }
        }
    }

    Ok(())
}
