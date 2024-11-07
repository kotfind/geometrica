use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{self, Error};

use crate::{
    result::{api_err, api_ok, ApiOk, ApiResult},
    App,
};

pub fn router() -> Router<App> {
    Router::new()
        .route("/get_all", post(get_all))
        .route("/get", post(get))
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

#[debug_handler(state = App)]
async fn get(
    State(App { scope, .. }): State<App>,
    Json(api::items::get::Request { name }): Json<api::items::get::Request>,
) -> ApiResult<api::items::get::Response> {
    let scope = scope.lock().await;
    let item = scope.get_item(&name);
    match item {
        Some(value) => Ok(api_ok(api::items::get::Response { value })),
        None => Err(api_err(Error {
            msg: format!("item {name} not found"),
        })),
    }
}
