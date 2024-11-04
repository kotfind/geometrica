use axum::{debug_handler, extract::State, routing::post, Json, Router};
use executor::exec::Exec;
use types::api::{ApiError, ExecRequest, ExecResponse};

use crate::{ApiResult, App};

pub fn router() -> Router<App> {
    Router::new().route("/", post(exec))
}

#[debug_handler(state = App)]
async fn exec(
    State(App { scope, .. }): State<App>,
    Json(ExecRequest { script }): Json<ExecRequest>,
) -> ApiResult<ExecResponse> {
    let script = parser::script(&script)
        .map_err(ApiError::from)
        .map_err(Json)?;

    let mut scope = scope.lock().await;

    script
        .exec(&mut scope)
        .map_err(ApiError::from)
        .map_err(Json)?;

    Ok(Json(ExecResponse))
}
