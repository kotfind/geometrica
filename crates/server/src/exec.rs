use axum::{debug_handler, extract::State, routing::post, Json, Router};
use executor::exec::Exec;
use types::api::{self};

use crate::{
    result::{api_err, api_ok, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(exec))
}

#[debug_handler(state = App)]
async fn exec(
    State(App { scope, .. }): State<App>,
    Json(api::exec::Request { script }): Json<api::exec::Request>,
) -> ApiResult<api::exec::Response> {
    let script = parser::script(&script).map_err(api_err)?;

    let mut scope = scope.lock().await;

    script.exec(&mut scope).map_err(api_err)?;

    Ok(api_ok(api::exec::Response))
}
