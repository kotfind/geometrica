use anyhow::Context;
use client::Client;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run(con: Client) -> anyhow::Result<()> {
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

                con.define(script_)
                    .await
                    .context("failed to execute script")?;

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

                con.define(line).await.context("failed to execute line")?;
            }
        }
    }

    Ok(())
}
