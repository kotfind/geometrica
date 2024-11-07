use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::App;

pub fn router() -> Router<App> {
    Router::new().route("/", get(ping))
}

#[debug_handler(state = App)]
async fn ping() -> impl IntoResponse {
    StatusCode::OK
}
