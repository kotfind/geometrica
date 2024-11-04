use std::sync::Arc;

use axum::Router;
use executor::exec::ExecScope;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::prelude::*;

mod eval;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let addr = "127.0.0.1:4242";
    info!("Server started on {addr}...");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router()).await?;

    Ok(())
}

fn router() -> Router {
    let app = App {
        scope: Arc::new(Mutex::new(ExecScope::new())),
    };

    Router::new().nest("/eval", eval::router()).with_state(app)
}

#[derive(Clone)]
struct App {
    scope: Arc<Mutex<ExecScope>>,
}
