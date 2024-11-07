use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::post, Router};

use crate::App;

pub fn router() -> Router<App> {
    Router::new().route("/", post(ping))
}

#[debug_handler(state = App)]
async fn ping() -> impl IntoResponse {
    StatusCode::OK
}
