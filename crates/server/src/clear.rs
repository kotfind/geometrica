use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self};

use crate::{
    result::{api_ok, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(clear))
}

#[debug_handler(state = App)]
async fn clear(
    State(App { scope, .. }): State<App>,
    Json(api::clear::Request {}): Json<api::clear::Request>,
) -> ApiResult<api::clear::Response> {
    let mut scope = scope.lock().await;

    scope.clear();

    Ok(api_ok(api::clear::Response {}))
}
