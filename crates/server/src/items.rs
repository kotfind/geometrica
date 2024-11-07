use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api;

use crate::{
    result::{api_ok, ApiOk},
    App,
};

pub fn router() -> Router<App> {
    Router::new().route("/get_all", post(get_all))
}

#[debug_handler(state = App)]
async fn get_all(
    State(App { scope, .. }): State<App>,
    Json(api::items::get_all::Request): Json<api::items::get_all::Request>,
) -> ApiOk<api::items::get_all::Response> {
    let scope = scope.lock().await;
    let items = scope.get_all_items();
    api_ok(api::items::get_all::Response { items })
}
