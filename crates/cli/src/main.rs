use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use client::{Client, ClientSettings};
use printing::ScriptResultPrinter;
use url::Url;

mod printing;
mod script_file_mode;
mod stdin_mode;
mod tty_mode;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    script_file: Option<PathBuf>,

    #[arg(long, default_value_t = Url::parse("http://127.0.0.1:4242").unwrap())]
    server_url: Url,

    #[arg(long, default_value_t = true)]
    do_init_server: bool,
    // TODO: server args
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = Client::from(ClientSettings {
        server_url: cli.server_url,
        try_spawn_server: cli.do_init_server,
    })
    .await
    .context("failed to connect to server")?;

    if let Some(script_file) = cli.script_file {
        script_file_mode::run(client, script_file).await?;
    } else if is_terminal::is_terminal(std::io::stdin()) {
        tty_mode::run(client).await?;
    } else {
        stdin_mode::run(client).await?;
    }

    Ok(())
}

async fn exec(client: &Client, script: impl ToString) -> anyhow::Result<()> {
    let res = client
        .exec(script.to_string())
        .await
        .context("failed to execute script");

    // TODO: Use writter
    print!("{}", ScriptResultPrinter(&res));

    match res.error {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
