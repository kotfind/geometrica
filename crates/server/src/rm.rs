use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self};

use crate::{
    result::{api_err, api_ok, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(rm))
}

#[debug_handler(state = App)]
async fn rm(
    State(App { scope, .. }): State<App>,
    Json(api::rm::Request { name }): Json<api::rm::Request>,
) -> ApiResult<api::rm::Response> {
    let mut scope = scope.lock().await;

    scope.rm(name).map_err(api_err)?;

    Ok(api_ok(api::rm::Response {}))
}
