use std::{io::Write, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::Context;
use axum::Router;
use clap::Parser;
use executor::exec::ExecScope;
use tempfile::NamedTempFile;
use tokio::{net::TcpListener, sync::Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod result;
mod routes;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Bind server to addr
    #[arg(long, default_value_t = SocketAddr::from_str("127.0.0.1:4242").unwrap())]
    bind: SocketAddr,

    /// Write listener port to <FILE>. Is usefull when binding to port 0.
    #[arg(long, value_name = "FILE")]
    port_file: Option<PathBuf>,
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
    if let Some(port_file) = cli.port_file {
        write_port(local_addr, port_file)?;
    }
    println!("Welcome to Geometrica Server!");
    println!("Listening on {}...", local_addr);

    axum::serve(listener, router()).await?;

    Ok(())
}

// TODO: delete file on close
fn write_port(local_addr: SocketAddr, port_file: PathBuf) -> anyhow::Result<()> {
    let tempfile = NamedTempFile::new_in({
        let mut dir = port_file.clone();
        dir.pop();
        dir
    })
    .context("failed to create tempfile")?;
    write!(tempfile.as_file(), "{}", local_addr.port()).with_context(|| {
        format!(
            "failed to write port to {}",
            tempfile.path().to_string_lossy()
        )
    })?;
    std::fs::rename(tempfile.path(), port_file.clone()).with_context(|| {
        format!(
            "failed to move {} to {}",
            tempfile.path().to_string_lossy(),
            port_file.to_string_lossy()
        )
    })?;
    Ok(())
}

fn router() -> Router {
    let app = App {
        scope: Arc::new(Mutex::new(ExecScope::new())),
    };

    routes::router().with_state(app)
}

#[derive(Clone)]
struct App {
    scope: Arc<Mutex<ExecScope>>,
}
