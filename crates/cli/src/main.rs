use std::{net::SocketAddr, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::Parser;
use client::{Client, ClientSettings};
use tabled::{builder::Builder, settings::Style};

mod script_file_mode;
mod stdin_mode;
mod tty_mode;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    script_file: Option<PathBuf>,

    // TODO: SocketAddr -> Url
    /// Server address
    #[arg(long, default_value_t = SocketAddr::from_str("127.0.0.1:4242").unwrap())]
    server_addr: SocketAddr,

    #[arg(long, default_value_t = true)]
    do_init_server: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let con = Client::from(ClientSettings {
        ip: cli.server_addr.ip(),
        port: cli.server_addr.port(),
        do_init_server: cli.do_init_server,
        ..Default::default()
    })
    .await
    .context("failed to connect to server")?;

    if let Some(script_file) = cli.script_file {
        script_file_mode::run(con, script_file).await?;
    } else if is_terminal::is_terminal(std::io::stdin()) {
        tty_mode::run(con).await?;
    } else {
        stdin_mode::run(con).await?;
    }

    Ok(())
}

async fn print_all_items(con: Client) -> anyhow::Result<()> {
    let mut items: Vec<_> = con
        .get_all_items()
        .await
        .context("failed to get all items")?
        .into_iter()
        .collect();
    items.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
    let mut table = Builder::new();
    table.push_record(["Name", "Value"]);
    for (name, value) in items {
        table.push_record([name.to_string(), value.to_string()]);
    }
    println!("{}", table.build().with(Style::sharp()));
    Ok(())
}
