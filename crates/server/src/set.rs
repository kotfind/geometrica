use std::collections::HashMap;

use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self};

use crate::{
    result::{api_err, api_ok, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(set))
}

#[debug_handler(state = App)]
async fn set(
    State(App { scope, .. }): State<App>,
    Json(api::set::Request { name, expr }): Json<api::set::Request>,
) -> ApiResult<api::set::Response> {
    let scope = scope.lock().await;

    let value = scope.eval_expr(expr, HashMap::new()).map_err(api_err)?;
    scope.set(name, value).map_err(api_err)?;

    Ok(api_ok(api::set::Response {}))
}
