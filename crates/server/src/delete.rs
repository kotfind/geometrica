use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self};

use crate::{
    result::{api_err, api_ok, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(delete))
}

#[debug_handler(state = App)]
async fn delete(
    State(App { scope, .. }): State<App>,
    Json(api::delete::Request { name }): Json<api::delete::Request>,
) -> ApiResult<api::delete::Response> {
    let mut scope = scope.lock().await;

    scope.delete(name).map_err(api_err)?;

    Ok(api_ok(api::delete::Response))
}
