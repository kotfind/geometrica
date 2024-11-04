use std::{net::SocketAddr, str::FromStr, sync::Arc};

use axum::{Json, Router};
use clap::Parser;
use executor::exec::ExecScope;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::prelude::*;
use types::api;

mod eval;
mod exec;
mod items;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Bind server to addr
    #[arg(long, default_value_t = SocketAddr::from_str("127.0.0.1:4242").unwrap())]
    bind: SocketAddr,

    /// Prints listener address to stderr in format 'listener_addr:{IP}:{PORT}'.
    /// Is mostly used in debugging purposes
    #[arg(long)]
    print_addr: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .without_time(),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let listener = TcpListener::bind(cli.bind).await?;
    let local_addr = listener.local_addr()?;
    if cli.print_addr {
        eprintln!("listener_addr:{}:{}", local_addr.ip(), local_addr.port());
    }
    info!("Listening on {}...", local_addr);

    axum::serve(listener, router()).await?;

    Ok(())
}

fn router() -> Router {
    let app = App {
        scope: Arc::new(Mutex::new(ExecScope::new())),
    };

    Router::new()
        .nest("/eval", eval::router())
        .nest("/exec", exec::router())
        .nest("/items", items::router())
        .with_state(app)
}

#[derive(Clone)]
struct App {
    scope: Arc<Mutex<ExecScope>>,
}

type ApiResult<T> = Result<Json<T>, Json<api::Error>>;
