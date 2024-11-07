use std::{net::SocketAddr, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::Parser;
use client::{Connection, ConnectionSettings};
use tabled::{builder::Builder, settings::Style};
use tokio::io::{AsyncBufReadExt, BufReader};

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

    let con = Connection::from(ConnectionSettings {
        ip: cli.server_addr.ip(),
        port: cli.server_addr.port(),
        do_init_server: cli.do_init_server,
        ..Default::default()
    })
    .await
    .context("failed to connect to server")?;

    let script = if let Some(script_file) = cli.script_file {
        tokio::fs::read_to_string(script_file)
            .await
            .context("failed to read script file")?
    } else {
        let mut script = String::new();
        let mut reader = BufReader::new(tokio::io::stdin());
        while reader
            .read_line(&mut script)
            .await
            .context("stdin read failed")?
            != 0
        {}
        script
    };

    con.exec(script).await.context("failed to execute script")?;

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
