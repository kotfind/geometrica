use std::{net::SocketAddr, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::Parser;
use client::{Client, ClientSettings, CommandResult};
use tabled::{
    builder::Builder,
    settings::{
        object::{Cell, Segment},
        Alignment, Modify, Span, Style, Width,
    },
};

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

fn print_table(from: client::Table) {
    // Create and fill table
    let mut to = Builder::new();

    to.push_record(from.header());

    if !from.is_empty() {
        for row in from.rows() {
            to.push_record(row);
        }
    } else {
        to.push_record(["*empty*"]);
    }

    // Apply styles
    let mut to = to.build();
    to.with(Style::rounded());

    if from.is_empty() {
        to.with(
            Modify::new(Cell::new(1, 0))
                .with(Span::column(from.width()))
                .with(Alignment::center()),
        );
    }

    if let Some(width) = termsize::get().map(|size| size.cols) {
        to.with(Modify::new(Segment::all()).with(Width::wrap(width as usize - from.width() * 6 /* XXX: magic constant that works for whatever reason */)));
    }

    // Print
    println!("{}", to)
}

fn print_command_res(res: CommandResult) {
    match res {
        CommandResult::Table(table) => print_table(table),
        CommandResult::Ok => println!("Ok"),
    }
}

async fn exec(client: &Client, script: impl ToString) -> anyhow::Result<()> {
    let res = client
        .exec(script.to_string())
        .await
        .context("failed to execute script")?;

    for res_line in res {
        print_command_res(res_line);
    }

    Ok(())
}
