use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api;

use crate::{
    result::{api_ok, ApiOk},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/list", post(list))
}

#[debug_handler(state = App)]
async fn list(
    State(App { scope, .. }): State<App>,
    Json(api::func::list::Request {}): Json<api::func::list::Request>,
) -> ApiOk<api::func::list::Response> {
    let scope = scope.lock().await;
    let (builtins, user_defined) = scope.list_funcs();
    api_ok(api::func::list::Response {
        builtins,
        user_defined,
    })
}
