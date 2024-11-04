use axum::{debug_handler, extract::State, routing::post, Json, Router};
use types::api::{GetAllItemsRequest, GetAllItemsResponse};

use crate::{ApiResult, App};

pub fn router() -> Router<App> {
    Router::new().route("/get_all", post(get_all))
}

#[debug_handler(state = App)]
async fn get_all(
    State(App { scope, .. }): State<App>,
    Json(GetAllItemsRequest): Json<GetAllItemsRequest>,
) -> ApiResult<GetAllItemsResponse> {
    let scope = scope.lock().await;
    let items = scope.get_all_items();
    Ok(Json(GetAllItemsResponse { items }))
}
