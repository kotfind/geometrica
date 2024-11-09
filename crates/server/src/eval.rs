use std::collections::HashMap;

use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self};

use crate::{
    result::{api_ok, ApiOk, IntoError},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/", post(eval))
}

#[debug_handler(state = App)]
async fn eval(
    State(App { scope, .. }): State<App>,
    Json(api::eval::Request { exprs }): Json<api::eval::Request>,
) -> ApiOk<api::eval::Response> {
    let mut values = Vec::with_capacity(exprs.len());

    for expr in exprs {
        values.push(
            scope
                .lock()
                .await
                .eval_expr(expr, HashMap::new())
                .map_err(IntoError::into_error),
        );
    }

    api_ok(api::eval::Response { values })
}
